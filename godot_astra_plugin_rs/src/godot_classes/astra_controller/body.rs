use crate::util;
use astra;
use gdnative::*;

#[derive(Default)]
pub struct BodyState {
    pub frame_index: i32,
    pub fps: u32,
}

impl super::AstraController {
    pub unsafe fn start_body_stream(&mut self, mut owner: Node) -> astra::Stream {
        let stream = astra::start_body_stream(self.reader);

        let mut body_timer = Timer::new();
        body_timer
            .connect(
                "timeout".into(),
                Some(*owner),
                "update_body".into(),
                VariantArray::new(),
                0,
            )
            .unwrap();

        body_timer.set_wait_time(1.0 / self.body_fps as f64);
        owner.add_child(Some(*body_timer), false);
        body_timer.start(0.0);
        stream
    }

    pub unsafe fn handle_update_body(&mut self, mut owner: Node) {
        astra::update();

        if let Some(mut frame) = astra::get_frame(self.reader) {
            self.handle_body_frame(&mut owner, frame);

            astra::close_frame(&mut frame);
        }
    }
    pub unsafe fn handle_body_frame(&mut self, owner: &mut Node, frame: astra::Frame) {
        let body_frame = astra::get_body_frame(frame);
        let body_frame_index = astra::get_body_frame_index(body_frame);

        if body_frame_index != self.body_frame_index {
            let bodies = astra::get_bodies(body_frame);
            let godot_bodies = body_list_to_variant_array(bodies);

            owner.emit_signal(
                GodotString::from_str("new_body_list"),
                &[Variant::from_array(&godot_bodies)],
            );
        }

        self.body_frame_index = body_frame_index;
    }
}

// average of 50usecs, this method is faster than json since json is slower in godot
fn body_list_to_variant_array(bodies: Vec<astra::Body>) -> VariantArray {
    let mut godot_bodies = VariantArray::new();

    for body in bodies
        .iter()
        .filter(|b| b.status == astra::BodyStatus::Tracking)
    {
        let mut godot_body = Dictionary::new();
        let mut godot_joints = Dictionary::new();

        for (joint_type, joint) in body
            .joints
            .iter()
            .filter(|(_, joint)| joint.status == astra::JointStatus::Tracked)
        {
            let mut godot_joint = Dictionary::new();
            let joint_type = &Variant::from(joint_type.clone() as u64);
            godot_joint.set(&Variant::from("joint_type"), &joint_type);
            godot_joint.set(
                &Variant::from("status"),
                &Variant::from(joint.status.clone() as u64),
            );
            godot_joint.set(
                &Variant::from("depth_position"),
                &Variant::from(&util::convert_vector2(&joint.depth_position)),
            );
            godot_joint.set(
                &Variant::from("world_position"),
                &Variant::from(&util::convert_vector3(&joint.world_position)),
            );
            godot_joints.set(&joint_type, &Variant::from_dictionary(&godot_joint));
        }

        godot_body.set(
            &Variant::from("joints"),
            &Variant::from_dictionary(&godot_joints),
        );

        godot_body.set(&Variant::from_str("id"), &Variant::from(body.id as u64));
        godot_body.set(
            &Variant::from_str("status"),
            &Variant::from(body.status.clone() as u64),
        );
        godot_body.set(
            &Variant::from_str("center_off_mass"),
            &Variant::from(&util::convert_vector3(&body.center_of_mass)),
        );
        godot_bodies.push(&Variant::from_dictionary(&godot_body));
    }
    godot_bodies
}
