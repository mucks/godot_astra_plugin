use astra;
use gdnative::init::{Property, PropertyHint, PropertyUsage};
use gdnative::*;

mod body;
mod color;
mod depth;
mod masked_color;

pub struct AstraController {
    sensor: Option<astra::Sensor>,
    body_fps: u32,
    color_fps: u32,
    depth_fps: u32,
    masked_color_fps: u32,
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
        builder.add_property(Property {
            name: "masked_color_fps",
            default: 30,
            hint: PropertyHint::Range {
                range: 0.0..60.0,
                step: 1.0,
                slider: true,
            },
            getter: |this: &AstraController| this.masked_color_fps,
            setter: |this: &mut AstraController, v| this.masked_color_fps = v,
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
            name: "new_color_img",
            args: &[init::SignalArgument {
                name: "image",
                default: Variant::from_object(&Image::new()),
                hint: init::PropertyHint::None,
                usage: init::PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(init::Signal {
            name: "new_masked_color_img",
            args: &[init::SignalArgument {
                name: "image",
                default: Variant::from_object(&Image::new()),
                hint: init::PropertyHint::None,
                usage: init::PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(init::Signal {
            name: "new_depth_img",
            args: &[init::SignalArgument {
                name: "image",
                default: Variant::from_object(&Image::new()),
                hint: init::PropertyHint::None,
                usage: init::PropertyUsage::DEFAULT,
            }],
        });
    }
}

#[methods]
impl AstraController {
    /// The "constructor" of the class.
    unsafe fn _init(_owner: Node) -> Self {
        let sensor = match astra::Sensor::new() {
            Ok(sensor) => Some(sensor),
            Err(err) => {
                godot_print!("{:?}", err);
                None
            }
        };
        AstraController {
            sensor: sensor,
            color_fps: 30,
            body_fps: 30,
            masked_color_fps: 30,
            depth_fps: 30,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node) {
        //self.start_body_stream(owner);
        self.start_depth_stream(owner);
        self.start_color_stream(owner);
        self.start_masked_color_stream(owner);
    }

    #[export]
    unsafe fn update_color(&mut self, owner: Node) {
        self.handle_update_color(owner);
    }

    #[export]
    unsafe fn update_body(&mut self, owner: Node) {
        self.handle_update_body(owner);
    }

    #[export]
    unsafe fn update_depth(&mut self, owner: Node) {
        self.handle_update_depth(owner)
    }

    #[export]
    unsafe fn update_masked_color(&mut self, owner: Node) {
        self.handle_update_masked_color(owner)
    }

    pub unsafe fn start_timer(&mut self, mut owner: Node, fps: u32, fn_name: &str) {
        let mut timer = Timer::new();
        timer
            .connect(
                "timeout".into(),
                Some(*owner),
                fn_name.into(),
                VariantArray::new(),
                0,
            )
            .unwrap();

        timer.set_wait_time(1.0 / fps as f64);
        owner.add_child(Some(*timer), false);
        timer.start(0.0);
    }
}
