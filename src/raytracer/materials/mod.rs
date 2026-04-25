use crate::lights::LightSample;
use crate::rendering::color::Color;
use crate::rendering::ray::HitRecord;

pub trait CanShade {
    fn shade(&self, hit: &HitRecord, light_samples: &[LightSample]) -> Color;
}

pub type Material = Box<dyn CanShade + Send + Sync>;
