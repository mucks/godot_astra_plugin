use crate::astra;
use crate::util::{astra_vec2_to_gd_vec2, astra_vec3_to_gd_vec3};
use astra::astra_bindings::astra_reader_frame_t;
use gdnative::init::{Property, PropertyHint, PropertyUsage};
use gdnative::*;

use num_cpus;

pub struct AstraController {
    reader: astra::astra_reader_t,
    body_frame_index: i32,
    color_frame_index: i32,
    color_fps: u32,
    body_fps: u32,
    rayon_time_diffs: Vec<i32>,
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
            getter: |this: &AstraController| this.color_fps,
            setter: |this: &mut AstraController, v| this.color_fps = v,
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
                    name: "color_base64",
                    default: Variant::from_byte_array(&ByteArray::new()),
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
            color_fps: 30,
            body_fps: 30,
            rayon_time_diffs: Vec::new(),
        }
    }
    #[export]
    unsafe fn _ready(&mut self, owner: Node) {
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

        color_timer.set_wait_time(1.0 / self.color_fps as f64);
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

    #[export]
    unsafe fn _process(&mut self, mut _owner: Node, _delta: f64) {}

    // I'm using base64 here because the ByteArray from godot was causing crashes
    unsafe fn handle_color_frame(&mut self, owner: &mut Node, frame: astra_reader_frame_t) {
        let color_frame = astra::get_color_frame(frame);
        let color_frame_index = astra::get_color_frame_index(color_frame);
        let thread_count = num_cpus::get();

        if color_frame_index != self.color_frame_index {
            let (width, height, color_data) = astra::get_color_bytes(color_frame);
            let rayon_time = std::time::SystemTime::now();

            let _ = color_data_to_color_byte_array_rayon(&color_data, thread_count);

            let rayon_time_mcs = rayon_time.elapsed().unwrap().as_micros() as i32;

            let without_rayon_time = std::time::SystemTime::now();
            let color_byte_array = color_data_to_color_byte_array(&color_data, thread_count);
            let without_rayon_time_mcs = without_rayon_time.elapsed().unwrap().as_micros() as i32;

            self.rayon_time_diffs
                .push(rayon_time_mcs - without_rayon_time_mcs);

            godot_print!(
                "time diff avg: {}",
                self.rayon_time_diffs.iter().sum::<i32>()
            );

            owner.emit_signal(
                GodotString::from_str("new_color_byte_array"),
                &[
                    Variant::from(width as u64),
                    Variant::from(height as u64),
                    Variant::from_byte_array(&color_byte_array),
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

use rayon::prelude::*;

fn color_data_to_color_byte_array_rayon(color_data: &Vec<u8>, thread_count: usize) -> ByteArray {
    let mut color_byte_array = ByteArray::new();
    let color_data_len = color_data.len();
    let data_chunks: Vec<&[u8]> = color_data.chunks(color_data_len / thread_count).collect();

    data_chunks
        .par_iter()
        .map(|chunk| {
            let mut byte_array = ByteArray::new();
            byte_array.resize(chunk.len() as i32);

            for i in 0..chunk.len() {
                byte_array.set(i as i32, chunk[i]);
            }
            byte_array
        })
        .collect::<Vec<ByteArray>>()
        .iter()
        .for_each(|byte_array| {
            color_byte_array.push_array(byte_array);
        });

    color_byte_array
}

fn color_data_to_color_byte_array(color_data: &Vec<u8>, thread_count: usize) -> ByteArray {
    let mut color_byte_array = ByteArray::new();
    let color_data_len = color_data.len();
    let data_chunks: Vec<&[u8]> = color_data.chunks(color_data_len / thread_count).collect();
    let mut handles = Vec::new();

    for data_chunk_ref in data_chunks {
        let data_chunk = data_chunk_ref.to_owned();

        handles.push(std::thread::spawn(move || {
            let mut byte_array = ByteArray::new();
            byte_array.resize(data_chunk.len() as i32);
            for i in 0..data_chunk.len() {
                byte_array.set(i as i32, data_chunk[i]);
            }
            byte_array
        }));
    }
    for handle in handles {
        color_byte_array.push_array(&handle.join().unwrap());
    }

    color_byte_array
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
