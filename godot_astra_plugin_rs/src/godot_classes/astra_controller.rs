use crate::astra;
use crate::util::{self, astra_vec2_to_gd_vec2, astra_vec3_to_gd_vec3};
use gdnative::*;

use base64::encode;

use astra::astra_bindings::astra_reader_frame_t;

use serde_json;

pub struct AstraController {
    reader: astra::astra_reader_t,
    body_frame_index: i32,
    color_frame_index: i32,
}

unsafe impl Send for AstraController {}

impl NativeClass for AstraController {
    type Base = Node;
    type UserData = user_data::MutexData<AstraController>;

    fn class_name() -> &'static str {
        "AstraController"
    }

    fn init(_owner: Self::Base) -> Self {
        unsafe { Self::_init(_owner) }
    }

    fn register_properties(builder: &init::ClassBuilder<Self>) {
        builder.add_signal(init::Signal {
            name: "new_body_list",
            args: &[init::SignalArgument {
                name: "body_list",
                default: Variant::from_array(&VariantArray::new()),
                hint: init::PropertyHint::None,
                usage: init::PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(init::Signal {
            name: "new_body_json",
            args: &[init::SignalArgument {
                name: "body_json",
                default: Variant::from(""),
                hint: init::PropertyHint::None,
                usage: init::PropertyUsage::DEFAULT,
            }],
        });

        // This event will cause editor crash sometimes
        builder.add_signal(init::Signal {
            name: "new_color_byte_array",
            args: &[
                init::SignalArgument {
                    name: "width",
                    default: Variant::from(0_u64),
                    hint: init::PropertyHint::None,
                    usage: init::PropertyUsage::DEFAULT,
                },
                init::SignalArgument {
                    name: "height",
                    default: Variant::from(0_u64),
                    hint: init::PropertyHint::None,
                    usage: init::PropertyUsage::DEFAULT,
                },
                init::SignalArgument {
                    name: "color_base64",
                    default: Variant::from(""),
                    hint: init::PropertyHint::None,
                    usage: init::PropertyUsage::DEFAULT,
                },
            ],
        });
    }
}

#[methods]
impl AstraController {
    /// The "constructor" of the class.
    unsafe fn _init(_owner: Node) -> Self {
        AstraController {
            reader: astra::init_sensor(),
            body_frame_index: -1,
            color_frame_index: -1,
        }
    }
    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are"attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&self, mut owner: Node) {
        astra::start_body_stream(self.reader);
        astra::start_color_stream(self.reader);

        let mut timer = Timer::new();
        timer
            .connect(
                "timeout".into(),
                Some(*owner),
                "astra_process".into(),
                VariantArray::new(),
                0,
            )
            .unwrap();

        let fps = 60.0;
        timer.set_wait_time(1.0 / fps);
        owner.add_child(Some(*timer), false);
        timer.start(0.0);
    }

    #[export]
    unsafe fn astra_process(&mut self, mut owner: Node) {
        astra::update();

        if let Some(mut frame) = astra::get_frame(self.reader) {
            self.handle_body_frame(&mut owner, frame);
            self.handle_color_frame(&mut owner, frame);

            astra::close_frame(&mut frame);
        }
    }

    #[export]
    unsafe fn _process(&mut self, mut owner: Node, delta: f64) {}

    // I'm using base64 here because the ByteArray from godot was causing crashes
    unsafe fn handle_color_frame(&mut self, owner: &mut Node, frame: astra_reader_frame_t) {
        let color_frame = astra::get_color_frame(frame);
        let color_frame_index = astra::get_color_frame_index(color_frame);

        if color_frame_index != self.color_frame_index {
            let (width, height, color_data) = astra::get_color_bytes(color_frame);
            let color_data_base64 = encode(&color_data);
            owner.emit_signal(
                GodotString::from_str("new_color_byte_array"),
                &[
                    Variant::from(width as u64),
                    Variant::from(height as u64),
                    Variant::from(&color_data_base64),
                ],
            );
        }
        self.color_frame_index = color_frame_index
    }

    unsafe fn handle_body_frame(&mut self, owner: &mut Node, frame: astra_reader_frame_t) {
        let body_frame = astra::get_body_frame(frame);
        let body_frame_index = astra::get_body_frame_index(body_frame);

        if body_frame_index != self.body_frame_index {
            let body_list = astra::get_body_list(body_frame);
            let t = std::time::SystemTime::now();

            let json_data = body_list_to_vec_body(&body_list);
            if let Ok(elapsed) = t.elapsed() {
                //godot_print!("body_json: {}", elapsed.as_micros());
            }
            owner.emit_signal(
                GodotString::from_str("new_body_json"),
                &[Variant::from(&json_data)],
            );

            let time = std::time::SystemTime::now();
            let godot_bodies = body_list_to_variant_array(&body_list);
            if let Ok(elapsed) = time.elapsed() {
                //godot_print!("body_hash: {}", elapsed.as_micros());
            }

            owner.emit_signal(
                GodotString::from_str("new_body_list"),
                &[Variant::from_array(&godot_bodies)],
            );
        }

        self.body_frame_index = body_frame_index;
    }
}

use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct MyVector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize)]
pub struct MyVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize)]
pub struct Joint {
    pub joint_type: u8,
    pub status: u8,
    pub depth_position: MyVector2,
    pub world_position: MyVector3,
}

#[derive(Serialize)]
pub struct Body {
    pub id: u8,
    pub status: u8,
    pub joints: HashMap<u8, Joint>,
    pub center_off_mass: MyVector3,
}

// takes an average of 30 microseconds to complete, testing showed it takes 30ms less than other option
fn body_list_to_vec_body(body_list: &astra::astra_bindings::_astra_body_list) -> String {
    let mut bodies = Vec::new();
    for i in 0..body_list.count {
        let body = body_list.bodies[i as usize];

        let mut joints = HashMap::new();

        for joint in body.joints.iter() {
            joints.insert(
                joint.type_,
                Joint {
                    joint_type: joint.type_,
                    status: joint.status,
                    depth_position: util::astra_vec2_to_my_vec2(&joint.depthPosition),
                    world_position: util::astra_vec3_to_my_vec3(&joint.worldPosition),
                },
            );
        }

        bodies.push(Body {
            id: body.id,
            status: body.status,
            joints: joints,
            center_off_mass: util::astra_vec3_to_my_vec3(&body.centerOfMass),
        });
    }
    serde_json::to_string(&bodies).unwrap()
}

// average of 50usecs, this method is faster than json since json is slower in godot
fn body_list_to_variant_array(body_list: &astra::astra_bindings::_astra_body_list) -> VariantArray {
    let mut godot_bodies = VariantArray::new();

    for i in 0..body_list.count {
        let mut godot_body = Dictionary::new();
        let mut godot_joints = Dictionary::new();
        let body = body_list.bodies[i as usize];

        for joint in body.joints.iter() {
            let mut godot_joint = Dictionary::new();
            let joint_type = &Variant::from(joint.type_ as u64);
            godot_joint.set(&Variant::from("joint_type"), &joint_type);
            godot_joint.set(
                &Variant::from("status"),
                &Variant::from(joint.status as u64),
            );
            godot_joint.set(
                &Variant::from("depth_position"),
                &Variant::from(&astra_vec2_to_gd_vec2(&joint.depthPosition)),
            );
            godot_joint.set(
                &Variant::from("world_position"),
                &Variant::from(&astra_vec3_to_gd_vec3(&joint.worldPosition)),
            );
            godot_joints.set(&joint_type, &Variant::from_dictionary(&godot_joint));
        }

        godot_body.set(
            &Variant::from("joints"),
            &Variant::from_dictionary(&godot_joints),
        );

        godot_body.set(&Variant::from_str("id"), &Variant::from(body.id as u64));
        godot_body.set(
            &Variant::from_str("status"),
            &Variant::from(body.status as u64),
        );
        godot_body.set(
            &Variant::from_str("center_off_mass"),
            &Variant::from(&astra_vec3_to_gd_vec3(&body.centerOfMass)),
        );
        godot_bodies.push(&Variant::from_dictionary(&godot_body));
    }
    godot_bodies
}
