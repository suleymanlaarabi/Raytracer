use crate::maths::vec3::{Position, Rotation, Vec3, cross};
use serde::Deserialize;

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

pub struct CameraBasis {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn basis(&self) -> CameraBasis {
        let pitch = self.rotation.x;
        let yaw = self.rotation.y;
        let forward = Vec3::from_xyz(
            pitch.cos() * yaw.sin(),
            -pitch.sin(),
            -pitch.cos() * yaw.cos(),
        )
        .normalize();
        let world_up = Vec3::from_xyz(0.0, 1.0, 0.0);
        let right = cross(forward, world_up).normalize();
        let up = cross(right, forward);
        CameraBasis { forward, right, up }
    }
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
