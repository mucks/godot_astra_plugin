pub fn convert_vector2(v: &astra::Vector2) -> gdnative::Vector2 {
    gdnative::Vector2::new(v.x, v.y)
}
pub fn convert_vector3(v: &astra::Vector3) -> gdnative::Vector3 {
    gdnative::Vector3::new(v.x, v.y, v.z)
}
