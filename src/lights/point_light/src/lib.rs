use raytracer::lights::{CanLight, Light, LightSample};
use raytracer::maths::vec3::Position;
use raytracer::rendering::color::ColorF;
use serde::Deserialize;

pub struct PointLight {
    position: Position,
    intensity: ColorF,
}

impl CanLight for PointLight {
    fn sample(&self, point: Position) -> LightSample {
        let delta = self.position - point;
        let distance = delta.length();
        LightSample {
            direction: delta / distance,
            distance,
            intensity: self.intensity,
        }
    }
}

#[derive(Deserialize)]
struct PointLightConfig {
    position: Position,
    intensity: (f32, f32, f32),
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Light {
    let config: PointLightConfig = cfg.clone().into_rust().expect("invalid config point_light");
    Box::new(PointLight {
        position: config.position,
        intensity: ColorF::from_rgb(config.intensity.0, config.intensity.1, config.intensity.2),
    })
}
