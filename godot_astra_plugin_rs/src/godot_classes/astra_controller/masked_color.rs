use gdnative::*;

impl super::AstraController {
    pub unsafe fn start_masked_color_stream(&mut self, owner: Node) {
        if let Some(sensor) = &mut self.sensor {
            match sensor.start_masked_color_stream() {
                Ok(_) => self.start_timer(owner, self.masked_color_fps, "update_masked_color"),
                Err(err) => godot_print!("{:?}", err),
            }
        }
    }

    pub unsafe fn handle_update_masked_color(&mut self, mut owner: Node) {
        if let Some(sensor) = &mut self.sensor {
            if sensor.update().is_ok() {
                if let Ok((width, height, byte_length, bytes)) = sensor.get_masked_color_bytes() {
                    let mut byte_array = ByteArray::new();
                    byte_array.resize(byte_length as i32);
                    std::ptr::copy(bytes.as_ptr(), byte_array.write().as_mut_ptr(), byte_length);

                    let mut img = Image::new();
                    img.create_from_data(width as i64, height as i64, false, 5, byte_array);
                    owner.emit_signal(
                        GodotString::from_str("new_masked_color_img"),
                        &[Variant::from_object(&img)],
                    );
                }
            }
        }
    }
}
