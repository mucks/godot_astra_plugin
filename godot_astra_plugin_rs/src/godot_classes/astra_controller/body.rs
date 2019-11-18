use gdnative::*;

impl super::AstraController {
    pub unsafe fn start_body_stream(&mut self, mut owner: Node) {
        match self.sensor.start_body_stream() {
            Ok(_) => self.start_timer(owner, self.body_fps, "update_body"),
            Err(err) => godot_print!("{:?}", err),
        }
    }

    pub unsafe fn handle_update_body(&mut self, mut owner: Node) {
        if let Ok(frame) = self.sensor.update() {
            let time = std::time::SystemTime::now();
            if let Ok(bodies) = self.sensor.get_bodies(&frame) {
                self.bodies = bodies;
                godot_print!("{}", time.elapsed().unwrap().as_micros());
            }
        }
    }
}
