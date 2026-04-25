use std::{
    fmt::{Display, Formatter},
    ops::*,
};

use ron::{Number, Value};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::from_xyz(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, t: f32) {
        *self = *self / t;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::from_xyz(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::from_xyz(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from_xyz(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from_xyz(self * v.x, self * v.y, self * v.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, t: f32) -> Vec3 {
        Vec3::from_xyz(self.x * t, self.y * t, self.z * t)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, t: f32) -> Vec3 {
        Vec3::from_xyz(self.x / t, self.y / t, self.z / t)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::from_xyz(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::splat(0.);

    #[inline]
    pub const fn splat(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }

    #[inline]
    pub fn from_ron_value(v: &Value) -> Option<Vec3> {
        let Value::Seq(s) = v else { return None };
        if s.len() != 3 {
            return None;
        }
        let f = |x: &Value| {
            if let Value::Number(n) = x {
                Some(n.into_f64() as f32)
            } else {
                None
            }
        };
        Some(Vec3::from_xyz(f(&s[0])?, f(&s[1])?, f(&s[2])?))
    }

    #[inline]
    pub fn to_value(&self) -> Value {
        let mut m = ron::Map::new();
        m.insert(
            Value::String("x".into()),
            Value::Number(Number::new(self.x)),
        );
        m.insert(
            Value::String("y".into()),
            Value::Number(Number::new(self.y)),
        );
        m.insert(
            Value::String("z".into()),
            Value::Number(Number::new(self.z)),
        );
        Value::Map(m)
    }

    #[inline]
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3::from_xyz(self.x / len, self.y / len, self.z / len)
    }

    #[inline]
    pub fn distance(&self, other: &Vec3) -> f32 {
        ((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z))
            .sqrt()
    }

    #[inline]
    pub fn project(&self, value: f32) -> Vec3 {
        Vec3::from_xyz(self.x * value, self.y * value, self.z * value)
    }

    #[inline]
    pub fn rotate(&self, rotation: Rotation) -> Vec3 {
        let rad_x = rotation.x.to_radians();
        let rad_y = rotation.y.to_radians();
        let rad_z = rotation.z.to_radians();

        let cos_x = rad_x.cos();
        let sin_x = rad_x.sin();
        let y1 = self.y * cos_x - self.z * sin_x;
        let z1 = self.y * sin_x + self.z * cos_x;
        let x1 = self.x;

        let cos_y = rad_y.cos();
        let sin_y = rad_y.sin();
        let x2 = x1 * cos_y + z1 * sin_y;
        let z2 = -x1 * sin_y + z1 * cos_y;
        let y2 = y1;

        let cos_z = rad_z.cos();
        let sin_z = rad_z.sin();
        let x3 = x2 * cos_z - y2 * sin_z;
        let y3 = x2 * sin_z + y2 * cos_z;
        let z3 = z2;

        Vec3::from_xyz(x3, y3, z3)
    }
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
}

pub type Translation = Vec3;
pub type Position = Vec3;
pub type Direction = Vec3;
pub type Rotation = Vec3;
pub type Scale = Vec3;
