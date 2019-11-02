use crate::astra;
use crate::util::{astra_vec2_to_gd_vec2, astra_vec3_to_gd_vec3};
use astra::astra_bindings::astra_reader_frame_t;
use gdnative::init::{Property, PropertyHint, PropertyUsage};
use gdnative::*;

use num_cpus;

pub struct AstraController {
    reader: astra::astra_reader_t,
    body_frame_index: i32,
    body_fps: u32,
    color: ColorState,
}

#[derive(Default)]
pub struct ColorState {
    frame_index: i32,
    fps: u32,
    byte_length: usize,
    width: u32,
    height: u32,
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
        builder.add_property(Property {
            name: "body_fps",
            default: 30,
            hint: PropertyHint::Range {
                range: 0.0..60.0,
                step: 1.0,
                slider: true,
            },
            getter: |this: &AstraController| this.body_fps,
            setter: |this: &mut AstraController, v| this.body_fps = v,
            usage: PropertyUsage::DEFAULT,
        });

        builder.add_property(Property {
            name: "color_fps",
            default: 30,
            hint: PropertyHint::Range {
                range: 0.0..60.0,
                step: 1.0,
                slider: true,
            },
            getter: |this: &AstraController| this.color.fps,
            setter: |this: &mut AstraController, v| this.color.fps = v,
            usage: PropertyUsage::DEFAULT,
        });

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
                    name: "image",
                    default: Variant::from_object(&Image::new()),
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
            color: ColorState {
                fps: 30,
                ..Default::default()
            },
            body_fps: 30,
        }
    }
    #[export]
    unsafe fn _ready(&mut self, owner: Node) {
        godot_print!("{}", self.color.fps);
        self.start_body_stream(owner);
        self.start_color_stream(owner);
    }

    unsafe fn start_body_stream(&mut self, mut owner: Node) {
        astra::start_body_stream(self.reader);

        let mut body_timer = Timer::new();
        body_timer
            .connect(
                "timeout".into(),
                Some(*owner),
                "update_body".into(),
                VariantArray::new(),
                0,
            )
            .unwrap();

        body_timer.set_wait_time(1.0 / self.body_fps as f64);
        owner.add_child(Some(*body_timer), false);
        body_timer.start(0.0);
    }

    unsafe fn start_color_stream(&mut self, mut owner: Node) {
        astra::start_color_stream(self.reader);

        let mut color_timer = Timer::new();
        color_timer
            .connect(
                "timeout".into(),
                Some(*owner),
                "update_color".into(),
                VariantArray::new(),
                0,
            )
            .unwrap();

        color_timer.set_wait_time(1.0 / self.color.fps as f64);
        owner.add_child(Some(*color_timer), false);
        color_timer.start(0.0);
    }

    #[export]
    unsafe fn update_body(&mut self, mut owner: Node) {
        astra::update();

        if let Some(mut frame) = astra::get_frame(self.reader) {
            self.handle_body_frame(&mut owner, frame);

            astra::close_frame(&mut frame);
        }
    }
    #[export]
    unsafe fn update_color(&mut self, mut owner: Node) {
        astra::update();

        if let Some(mut frame) = astra::get_frame(self.reader) {
            self.handle_color_frame(&mut owner, frame);

            astra::close_frame(&mut frame);
        }
    }

    unsafe fn handle_color_frame(&mut self, owner: &mut Node, frame: astra_reader_frame_t) {
        let color_frame = astra::get_color_frame(frame);
        let color_frame_index = astra::get_color_frame_index(color_frame);

        if color_frame_index != self.color.frame_index {
            let byte_length = astra::get_color_frame_byte_length(color_frame);
            let mut byte_array = ByteArray::new();
            byte_array.resize(byte_length as i32);
            if self.color.byte_length != byte_length {
                self.color.byte_length = byte_length;
                let (width, height) = astra::get_color_frame_dimensions(color_frame);
                self.color.width = width;
                self.color.height = height;
            }
            astra::get_color_byte_array(color_frame, byte_array.write().as_mut_ptr());
            let mut img = Image::new();
            img.create_from_data(
                self.color.width as i64,
                self.color.height as i64,
                false,
                4,
                byte_array,
            );

            owner.emit_signal(
                GodotString::from_str("new_color_byte_array"),
                &[
                    Variant::from(self.color.width as u64),
                    Variant::from(self.color.height as u64),
                    Variant::from_object(&img),
                ],
            );
        }
        self.color.frame_index = color_frame_index
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
