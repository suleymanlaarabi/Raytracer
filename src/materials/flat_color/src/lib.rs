use raytracer::lights::LightSample;
use raytracer::materials::{CanShade, Material};
use raytracer::maths::vec3::dot;
use raytracer::rendering::color::{Color, ColorF};
use raytracer::rendering::ray::HitRecord;
use serde::Deserialize;

pub struct FlatColor {
    color: ColorF,
    ambient: f32,
}

impl CanShade for FlatColor {
    fn shade(&self, hit: &HitRecord, light_samples: &[LightSample]) -> Color {
        let mut r = self.color.r * self.ambient;
        let mut g = self.color.g * self.ambient;
        let mut b = self.color.b * self.ambient;

        for sample in light_samples {
            let cos_theta = dot(hit.normal, sample.direction).max(0.0);
            r += self.color.r * sample.intensity.r * cos_theta;
            g += self.color.g * sample.intensity.g * cos_theta;
            b += self.color.b * sample.intensity.b * cos_theta;
        }

        Color::from_colorf(ColorF::from_rgb(r, g, b))
    }
}

#[derive(Deserialize)]
struct FlatColorConfig {
    color: (u8, u8, u8),
    #[serde(default = "default_ambient")]
    ambient: f32,
}

fn default_ambient() -> f32 {
    0.1
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Material {
    let config: FlatColorConfig = cfg.clone().into_rust().expect("invalid flat_color config");
    Box::new(FlatColor {
        color: ColorF::from_u8(config.color.0, config.color.1, config.color.2),
        ambient: config.ambient,
    })
}
