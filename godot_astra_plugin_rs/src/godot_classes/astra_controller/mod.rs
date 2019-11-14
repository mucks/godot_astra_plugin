use astra;
use astra::StreamType;
use gdnative::*;
use std::collections::HashMap;

mod body;
mod img;
mod native_class;

pub struct AstraController {
    sensor: astra::Sensor,
    body_fps: u32,
    color_fps: u32,
    depth_fps: u32,
    masked_color_fps: u32,
    color_enabled: bool,
    body_enabled: bool,
    depth_enabled: bool,
    masked_color_enabled: bool,
    bodies: VariantArray,
    images: HashMap<astra::StreamType, Image>,
}

unsafe impl Send for AstraController {}

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
            bodies: VariantArray::new(),
            color_enabled: false,
            masked_color_enabled: false,
            body_enabled: false,
            depth_enabled: false,
            images: [
                (StreamType::Color, Image::new()),
                (StreamType::MaskedColor, Image::new()),
                (StreamType::Depth, Image::new()),
                (StreamType::Infrared, Image::new()),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node) {
        self.sensor.init().unwrap();
        if self.color_enabled {
            self.start_color_stream(owner);
        }
        if self.body_enabled {
            self.start_body_stream(owner);
        }
        if self.masked_color_enabled {
            self.start_masked_color_stream(owner);
        }
        if self.depth_enabled {
            self.start_depth_stream(owner);
        }
    }

    #[export]
    pub unsafe fn get_color_image(&self, owner: Node) -> Image {
        self.images[&StreamType::Color].new_ref()
    }

    #[export]
    pub unsafe fn get_masked_color_image(&self, owner: Node) -> Image {
        self.images[&StreamType::MaskedColor].new_ref()
    }
    #[export]
    pub unsafe fn get_depth_image(&self, owner: Node) -> Image {
        self.images[&StreamType::Depth].new_ref()
    }

    #[export]
    pub unsafe fn get_bodies(&self, owner: Node) -> VariantArray {
        self.bodies.new_ref()
    }

    #[export]
    unsafe fn update_color(&mut self, owner: Node) {
        self.handle_update_img(owner, StreamType::Color);
    }

    #[export]
    unsafe fn update_body(&mut self, owner: Node) {
        self.handle_update_body(owner);
    }

    #[export]
    unsafe fn update_depth(&mut self, owner: Node) {
        self.handle_update_img(owner, StreamType::Depth)
    }

    #[export]
    unsafe fn update_masked_color(&mut self, owner: Node) {
        self.handle_update_img(owner, StreamType::MaskedColor)
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
    pub unsafe fn start_depth_stream(&mut self, owner: Node) {
        match self.sensor.start_depth_stream() {
            Ok(_) => self.start_timer(owner, self.depth_fps, "update_depth"),
            Err(err) => godot_print!("{:?}", err),
        }
    }
    pub unsafe fn start_color_stream(&mut self, owner: Node) {
        match self.sensor.start_color_stream() {
            Ok(_) => self.start_timer(owner, self.color_fps, "update_color"),
            Err(err) => godot_print!("{:?}", err),
        }
    }
    pub unsafe fn start_masked_color_stream(&mut self, owner: Node) {
        match self.sensor.start_masked_color_stream() {
            Ok(_) => self.start_timer(owner, self.masked_color_fps, "update_masked_color"),
            Err(err) => godot_print!("{:?}", err),
        }
    }
}
