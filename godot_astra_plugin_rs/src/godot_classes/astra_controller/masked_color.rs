use gdnative::*;

impl super::AstraController {
    pub unsafe fn start_masked_color_stream(&mut self, owner: Node) {
        match self.sensor.start_masked_color_stream() {
            Ok(_) => self.start_timer(owner, self.masked_color_fps, "update_masked_color"),
            Err(err) => godot_print!("{:?}", err),
        }
    }

    pub unsafe fn handle_update_masked_color(&mut self, mut owner: Node) {
        if let Ok(frame) = self.sensor.update() {
            if let Ok((width, height, byte_length, bytes)) =
                self.sensor.get_masked_color_bytes(&frame)
            {
                let mut byte_array = ByteArray::new();
                byte_array.resize(byte_length as i32);
                std::ptr::copy(bytes.as_ptr(), byte_array.write().as_mut_ptr(), byte_length);

                self.masked_color_image.create_from_data(
                    width as i64,
                    height as i64,
                    false,
                    5,
                    byte_array,
                );
            }
        }
    }
}
