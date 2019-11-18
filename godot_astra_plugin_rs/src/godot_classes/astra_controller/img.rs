use astra::StreamType;
use gdnative::*;

impl super::AstraController {
    pub unsafe fn handle_update_img(&mut self, mut owner: Node, stream_type: astra::StreamType) {
        if let Ok(frame) = self.sensor.update() {
            if let Ok(img_frame) = frame.get_img_frame(stream_type) {
                if self
                    .sensor
                    .has_new_frame(&img_frame, stream_type)
                    .unwrap_or(false)
                {
                    if let Ok((width, height)) = self.sensor.get_img_dimensions(&img_frame) {
                        if let Ok(byte_length) = self.sensor.get_img_byte_length(&img_frame) {
                            let mut byte_array = ByteArray::new();
                            byte_array.resize(byte_length as i32);

                            self.sensor
                                .copy_img_bytes(&img_frame, byte_array.write().as_mut_ptr())
                                .unwrap();

                            self.images.get_mut(&stream_type).unwrap().create_from_data(
                                width as i64,
                                height as i64,
                                false,
                                match stream_type {
                                    StreamType::Color => 4,
                                    StreamType::MaskedColor => 5,
                                    StreamType::Depth => 0,
                                    _ => 0,
                                },
                                byte_array,
                            );
                        }
                    }
                }
            }
        }
    }
}
