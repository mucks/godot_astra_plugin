use astra;
use gdnative::*;

#[derive(Default)]
pub struct ColorState {
    pub frame_index: i32,
    pub fps: u32,
    pub byte_length: usize,
    pub width: u32,
    pub height: u32,
    pub masked: bool,
}

impl super::AstraController {
    pub unsafe fn start_masked_color_stream(&mut self, mut owner: Node) {
        astra::start_masked_color_stream(self.reader);

        let mut color_timer = Timer::new();
        color_timer
            .connect(
                "timeout".into(),
                Some(*owner),
                "update_masked_color".into(),
                VariantArray::new(),
                0,
            )
            .unwrap();

        color_timer.set_wait_time(1.0 / self.color.fps as f64);
        owner.add_child(Some(*color_timer), false);
        color_timer.start(0.0);
    }
    pub unsafe fn handle_update_masked_color(&mut self, mut owner: Node) {
        astra::update();

        if let Some(mut frame) = astra::get_frame(self.reader) {
            self.handle_masked_color_frame(&mut owner, frame);
            astra::close_frame(&mut frame);
        }
    }
    pub unsafe fn handle_masked_color_frame(&mut self, owner: &mut Node, frame: astra::Frame) {
        let color_frame = astra::get_masked_color_frame(frame);
        let color_frame_index = astra::get_masked_color_frame_index(color_frame);

        if color_frame_index != self.color.frame_index {
            let byte_length = astra::get_masked_color_frame_byte_length(color_frame);
            let mut byte_array = ByteArray::new();
            byte_array.resize(byte_length as i32);
            if self.color.byte_length != byte_length {
                self.color.byte_length = byte_length;
                let (width, height) = astra::get_masked_color_frame_dimensions(color_frame);
                self.color.width = width;
                self.color.height = height;
            }
            astra::get_masked_color_byte_array(color_frame, byte_array.write().as_mut_ptr());
            astra::get_color_byte_array(color_frame, byte_array.write().as_mut_ptr());
            let mut img = Image::new();
            img.create_from_data(
                self.color.width as i64,
                self.color.height as i64,
                false,
                5,
                byte_array,
            );

            owner.emit_signal(
                GodotString::from_str("new_color_byte_array"),
                &[
                    Variant::from(self.color.width as u64),
                    Variant::from(self.color.height as u64),
                    Variant::from_object(&img),
                ],
            );
        }
        self.color.frame_index = color_frame_index
    }
}
