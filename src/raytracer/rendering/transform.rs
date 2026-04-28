use ron::{Map, Value};
use serde::{Deserialize, Deserializer};

use crate::maths::vec3::{Rotation, Translation, Vec3};

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Translation,
    pub rotation: Rotation,
    pub rot_mat: [[f32; 3]; 3],
}

fn compute_rot_mat(r: Vec3) -> [[f32; 3]; 3] {
    let (sx, cx) = r.x.to_radians().sin_cos();
    let (sy, cy) = r.y.to_radians().sin_cos();
    let (sz, cz) = r.z.to_radians().sin_cos();
    [
        [cz * cy, cz * sy * sx - sz * cx, cz * sy * cx + sz * sx],
        [sz * cy, sz * sy * sx + cz * cx, sz * sy * cx - cz * sx],
        [-sy, cy * sx, cy * cx],
    ]
}

impl<'de> Deserialize<'de> for Transform {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            translation: Option<Vec3>,
            rotation: Option<Vec3>,
        }
        let r = Raw::deserialize(d)?;
        Ok(Transform::new(
            r.translation.unwrap_or(Vec3::ZERO),
            r.rotation.unwrap_or(Vec3::ZERO),
        ))
    }
}

impl Transform {
    pub const ZERO: Transform = Transform {
        translation: Vec3::ZERO,
        rotation: Vec3::ZERO,
        rot_mat: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };

    pub fn new(translation: Translation, rotation: Rotation) -> Self {
        Transform {
            translation,
            rotation,
            rot_mat: compute_rot_mat(rotation),
        }
    }

    #[inline]
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Transform::new(Translation::from_xyz(x, y, z), Vec3::ZERO)
    }

    #[inline]
    pub fn rotate_vec(&self, v: Vec3) -> Vec3 {
        let m = &self.rot_mat;
        Vec3::from_xyz(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
        )
    }

    pub fn to_map(&self) -> Map {
        [
            ("translation", self.translation),
            ("rotation", self.rotation),
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
