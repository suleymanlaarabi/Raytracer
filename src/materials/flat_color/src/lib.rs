use raytracer::material::{CanShade, Material};
use raytracer::rendering::color::Color;
use raytracer::rendering::ray::HitRecord;
use serde::Deserialize;

pub struct FlatColor {
    r: u8,
    g: u8,
    b: u8,
}

impl CanShade for FlatColor {
    fn shade(&self, _hit: &HitRecord) -> Color {
        Color::from_rgb(self.r, self.g, self.b)
    }
}

#[derive(Deserialize)]
struct FlatColorConfig {
    color: (u8, u8, u8),
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Material {
    let config: FlatColorConfig = cfg.clone().into_rust().expect("invalid flat_color config");
    Box::new(FlatColor {
        r: config.color.0,
        g: config.color.1,
        b: config.color.2,
    })
}
