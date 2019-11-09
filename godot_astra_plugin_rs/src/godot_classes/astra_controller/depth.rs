use gdnative::*;

impl super::AstraController {
    pub unsafe fn start_depth_stream(&mut self, owner: Node) {
        if let Some(sensor) = &mut self.sensor {
            match sensor.start_depth_stream() {
                Ok(_) => self.start_timer(owner, self.depth_fps, "update_depth"),
                Err(err) => godot_print!("{:?}", err),
            }
        }
    }

    pub unsafe fn handle_update_depth(&mut self, mut owner: Node) {
        if let Some(sensor) = &mut self.sensor {
            if sensor.update().is_ok() {
                if let Ok((width, height, byte_length, bytes)) = sensor.get_depth_bytes() {
                    let mut byte_array = ByteArray::new();
                    byte_array.resize(byte_length as i32);
                    std::ptr::copy(bytes.as_ptr(), byte_array.write().as_mut_ptr(), byte_length);
                    let mut img = Image::new();

                    img.create_from_data(width as i64, height as i64, false, 0, byte_array);
                    owner.emit_signal(
                        GodotString::from_str("new_depth_img"),
                        &[Variant::from_object(&img)],
                    );
                }
            }
        }
    }
}
