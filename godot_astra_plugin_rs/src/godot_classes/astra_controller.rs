use crate::astra;
use crate::util::{astra_vec2_to_gd_vec2, astra_vec3_to_gd_vec3};
use astra::astra_bindings::astra_reader_frame_t;
use base64::encode;
use gdnative::*;

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
            //self.handle_color_frame(&mut owner, frame);

            astra::close_frame(&mut frame);
        }
    }

    #[export]
    unsafe fn _process(&mut self, mut owner: Node, delta: f64) {}

    // I'm using base64 here because the ByteArray from godot was causing crashes
    // TODO: this is not working on android for some reason
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
            let godot_bodies = body_list_to_variant_array(&body_list);

            owner.emit_signal(
                GodotString::from_str("new_body_list"),
                &[Variant::from_array(&godot_bodies)],
            );
        }

        self.body_frame_index = body_frame_index;
    }
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
