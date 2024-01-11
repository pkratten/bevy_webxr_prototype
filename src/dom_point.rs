use bevy::math::{Quat, Vec3};
use web_sys::DomPointReadOnly;

pub fn dom_point_to_vec3(point: &DomPointReadOnly) -> Vec3 {
    Vec3 {
        x: point.x() as f32,
        y: point.y() as f32,
        z: point.z() as f32,
    }
}

pub fn dom_point_to_quat(point: &DomPointReadOnly) -> Quat {
    Quat {
        x: point.x() as f32,
        y: point.y() as f32,
        z: point.z() as f32,
        w: point.w() as f32,
    }
}
