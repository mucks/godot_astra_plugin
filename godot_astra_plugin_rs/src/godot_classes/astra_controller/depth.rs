use gdnative::*;

impl super::AstraController {
    pub unsafe fn start_depth_stream(&mut self, owner: Node) {
        match self.sensor.start_depth_stream() {
            Ok(_) => self.start_timer(owner, self.depth_fps, "update_depth"),
            Err(err) => godot_print!("{:?}", err),
        }
    }

    pub unsafe fn handle_update_depth(&mut self, mut owner: Node) {
        if let Ok(frame) = self.sensor.update() {
            if let Ok((width, height, byte_length, bytes)) = self.sensor.get_depth_bytes(&frame) {
                let mut byte_array = ByteArray::new();
                byte_array.resize(byte_length as i32);
                std::ptr::copy(bytes.as_ptr(), byte_array.write().as_mut_ptr(), byte_length);

                self.depth_image.create_from_data(
                    width as i64,
                    height as i64,
                    false,
                    0,
                    byte_array,
                );
            }
        }
    }
}
