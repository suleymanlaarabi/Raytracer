use crate::rendering::color::Color;
use crate::rendering::ray::HitRecord;

pub trait CanShade {
    fn shade(&self, hit: &HitRecord) -> Color;
}

pub type Material = Box<dyn CanShade + Send + Sync>;
