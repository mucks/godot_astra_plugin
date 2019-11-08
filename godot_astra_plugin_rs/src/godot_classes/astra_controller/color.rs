use gdnative::*;

impl super::AstraController {
    pub unsafe fn start_color_stream(&mut self, mut owner: Node) {
        if let Some(sensor) = &mut self.sensor {
            godot_print!("color_stream: {:?}", sensor.start_color_stream());

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
    }

    pub unsafe fn handle_update_color(&mut self, mut owner: Node) {
        if let Some(sensor) = &mut self.sensor {
            if let Ok(frame) = sensor.update_color() {
                let meta = frame.color_meta.unwrap();
                let mut img = Image::new();
                let mut byte_array = ByteArray::new();
                byte_array.resize(meta.byte_length as i32);
                let byte_ptr = byte_array.write().as_mut_ptr();

                if let Ok(_) = sensor.get_color_ptr(frame, byte_ptr) {
                    img.create_from_data(
                        meta.width as i64,
                        meta.height as i64,
                        false,
                        4,
                        byte_array,
                    );
                    owner.emit_signal(
                        GodotString::from_str("new_color_img"),
                        &[Variant::from_object(&img)],
                    );
                }
            }
        }
    }
}
