use astra;
use astra::StreamType;
use astra::StreamType::*;
use gdnative::*;
use std::collections::HashMap;

mod native_class;

pub struct AstraController {
    sensor: astra::Sensor,
    color_enabled: bool,
    body_enabled: bool,
    depth_enabled: bool,
    masked_color_enabled: bool,
    bodies: Vec<astra::Body>,
    images: HashMap<astra::StreamType, Image>,
}

unsafe impl Send for AstraController {}

#[methods]
impl AstraController {
    /// The "constructor" of the class.
    unsafe fn _init(_owner: Node) -> Self {
        AstraController {
            sensor: astra::Sensor::new(),
            bodies: Vec::new(),
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
            self.sensor.start_color_stream().unwrap();
        }
        if self.body_enabled {
            self.sensor.start_body_stream().unwrap();
        }
        if self.masked_color_enabled {
            self.sensor.start_masked_color_stream().unwrap();
        }
        if self.depth_enabled {
            self.sensor.start_depth_stream().unwrap();
        }
    }

    #[export]
    unsafe fn _process(&mut self, owner: Node, delta: f64) {
        if let Ok(frame) = self.sensor.update() {
            if self.color_enabled {
                if let Ok(img) = self.sensor.get_img(&frame, Color) {
                    *self.images.get_mut(&Color).unwrap() = img;
                }
            }
            if self.masked_color_enabled {
                if let Ok(img) = self.sensor.get_img(&frame, MaskedColor) {
                    *self.images.get_mut(&MaskedColor).unwrap() = img;
                }
            }
            if self.depth_enabled {
                if let Ok(img) = self.sensor.get_img(&frame, Depth) {
                    *self.images.get_mut(&MaskedColor).unwrap() = img;
                }
            }
            if self.body_enabled {
                if let Ok(bodies) = self.sensor.get_bodies(&frame) {
                    self.bodies = bodies;
                }
            }
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
    pub unsafe fn get_bodies(&self, owner: Node) -> gdnative::Variant {
        self.bodies.to_variant()
    }
}
