use ron::{Map, Value};
use serde::{Deserialize, Deserializer};

use crate::maths::vec3::{Rotation, Scale, Translation, Vec3};

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl<'de> Deserialize<'de> for Transform {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            translation: Option<Vec3>,
            rotation: Option<Vec3>,
            scale: Option<Vec3>,
        }
        let r = Raw::deserialize(d)?;
        Ok(Transform {
            translation: r.translation.unwrap_or(Vec3::ZERO),
            rotation: r.rotation.unwrap_or(Vec3::ZERO),
            scale: r.scale.unwrap_or(Vec3::ZERO),
        })
    }
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

    pub fn to_map(&self) -> Map {
        [
            ("translation", self.translation),
            ("rotation", self.rotation),
            ("scale", self.scale),
        ]
        .into_iter()
        .map(|(k, v)| (Value::String(k.into()), v.to_value()))
        .collect()
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
