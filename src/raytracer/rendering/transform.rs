use serde::Deserialize;

use crate::maths::vec3::{Rotation, Scale, Translation, Vec3};

#[derive(Deserialize)]
pub struct Transform {
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Transform {
    pub const ZERO: Transform = Transform::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);

    #[inline]
    pub const fn new(translation: Translation, rotation: Rotation, scale: Scale) -> Self {
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    #[inline]
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Transform {
            translation: Translation::from_xyz(x, y, z),
            rotation: Vec3::ZERO,
            scale: Vec3::ZERO,
        }
    }

    #[inline]
    pub fn with_translation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    #[inline]
    pub fn with_rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    #[inline]
    pub fn with_scale(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }
}
