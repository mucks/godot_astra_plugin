use astra;
use gdnative::init::{Property, PropertyHint, PropertyUsage};
use gdnative::*;

mod body;
mod color;
mod depth;
mod masked_color;

pub struct AstraController {
    sensor: astra::Sensor,
    body_fps: u32,
    color_fps: u32,
    depth_fps: u32,
    masked_color_fps: u32,
    color_image: Image,
    masked_color_image: Image,
    depth_image: Image,
    bodies: VariantArray,
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
    }
}

#[methods]
impl AstraController {
    /// The "constructor" of the class.
    unsafe fn _init(_owner: Node) -> Self {
        AstraController {
            sensor: astra::Sensor::new(),
            color_fps: 30,
            body_fps: 30,
            masked_color_fps: 30,
            depth_fps: 30,
            color_image: Image::new(),
            masked_color_image: Image::new(),
            depth_image: Image::new(),
            bodies: VariantArray::new(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node) {
        self.sensor.init().unwrap();
        self.start_color_stream(owner);
        self.start_body_stream(owner);
        self.start_depth_stream(owner);
        self.start_masked_color_stream(owner);
    }

    #[export]
    pub unsafe fn get_color_image(&self, owner: Node) -> Image {
        self.color_image.new_ref()
    }

    #[export]
    pub unsafe fn get_masked_color_image(&self, owner: Node) -> Image {
        self.masked_color_image.new_ref()
    }
    #[export]
    pub unsafe fn get_depth_image(&self, owner: Node) -> Image {
        self.depth_image.new_ref()
    }

    #[export]
    pub unsafe fn get_bodies(&self, owner: Node) -> VariantArray {
        self.bodies.new_ref()
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
