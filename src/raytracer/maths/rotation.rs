use crate::maths::vec3::{Rotation, Vec3};

#[inline]
pub fn rotate_x(v: Vec3, angle: f32) -> Vec3 {
    let (s, c) = angle.sin_cos();
    Vec3::from_xyz(v.x, v.y * c - v.z * s, v.y * s + v.z * c)
}

#[inline]
pub fn rotate_y(v: Vec3, angle: f32) -> Vec3 {
    let (s, c) = angle.sin_cos();
    Vec3::from_xyz(v.x * c + v.z * s, v.y, -v.x * s + v.z * c)
}

#[inline]
pub fn rotate(v: Vec3, rot: Rotation) -> Vec3 {
    rotate_x(rotate_y(v, rot.y), rot.x)
}

#[inline]
pub fn rotate_inverse(v: Vec3, rot: Rotation) -> Vec3 {
    rotate_y(rotate_x(v, -rot.x), -rot.y)
}
