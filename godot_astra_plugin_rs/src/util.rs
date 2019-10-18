use crate::astra::astra_bindings::{astra_vector2f_t, astra_vector3f_t};
use crate::godot_classes::astra_controller::{MyVector2, MyVector3};
use gdnative::{Vector2, Vector3};

pub fn astra_vec3_to_gd_vec3(vector: &astra_vector3f_t) -> Vector3 {
    Vector3::new(vector.x, vector.y, vector.z)
}

pub fn astra_vec3_to_my_vec3(vector: &astra_vector3f_t) -> MyVector3 {
    MyVector3 {
        x: vector.x,
        y: vector.y,
        z: vector.z,
    }
}

pub fn astra_vec2_to_gd_vec2(vector: &astra_vector2f_t) -> Vector2 {
    Vector2::new(vector.x, vector.y)
}

pub fn astra_vec2_to_my_vec2(vector: &astra_vector2f_t) -> MyVector2 {
    MyVector2 {
        x: vector.x,
        y: vector.y,
    }
}
