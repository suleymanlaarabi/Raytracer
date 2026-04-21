use serde::Deserialize;

use crate::vec3::{Position, Rotation, Vec3};

#[derive(Deserialize, Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Debug)]
pub struct Camera {
    pub position: Position,
    pub rotation: Rotation,
    pub resolution: Resolution,
    pub fov: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Position::ZERO,
            rotation: Vec3::ZERO,
            resolution: Resolution {
                width: 720,
                height: 720,
            },
            fov: 1.,
        }
    }
}
