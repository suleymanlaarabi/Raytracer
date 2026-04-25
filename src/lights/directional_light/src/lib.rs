use raytracer::lights::{CanLight, Light, LightSample};
use raytracer::maths::vec3::{Direction, Position};
use raytracer::rendering::color::ColorF;
use serde::Deserialize;

pub struct DirectionalLight {
    toward_light: Direction,
    intensity: ColorF,
}

impl CanLight for DirectionalLight {
    fn sample(&self, _point: Position) -> LightSample {
        LightSample {
            direction: self.toward_light,
            distance: f32::INFINITY,
            intensity: self.intensity,
        }
    }
}

#[derive(Deserialize)]
struct DirectionalLightConfig {
    direction: Direction,
    intensity: (f32, f32, f32),
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Light {
    let config: DirectionalLightConfig = cfg
        .clone()
        .into_rust()
        .expect("invalid config directional_light");
    Box::new(DirectionalLight {
        toward_light: config.direction.normalize(),
        intensity: ColorF::from_rgb(config.intensity.0, config.intensity.1, config.intensity.2),
    })
}
