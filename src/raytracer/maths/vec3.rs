use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::from_xyz(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

impl DivAssign<f32> for Vec3 {
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

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::from_xyz(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from_xyz(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

// f32 * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from_xyz(self * v.x, self * v.y, self * v.z)
    }
}

// Vec3 * f32
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3::from_xyz(self.x * t, self.y * t, self.z * t)
    }
}

// Vec3 / f32
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3::from_xyz(self.x / t, self.y / t, self.z / t)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::from_xyz(0., 0., 0.);

    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3::from_xyz(self.x / len, self.y / len, self.z / len)
    }

    pub fn distance(&self, other: &Vec3) -> f32 {
        ((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z))
            .sqrt()
    }

    pub fn project(&self, value: f32) -> Vec3 {
        Vec3::from_xyz(self.x * value, self.y * value, self.z * value)
    }
}

pub type Position = Vec3;
pub type Direction = Vec3;
pub type Rotation = Vec3;
