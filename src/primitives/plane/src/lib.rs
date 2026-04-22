use raytracer::primitives::Primitive;
use serde::Deserialize;

use raytracer::maths::vec3::{Position, Vec3, dot};
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};

pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
}

impl CanHit for Plane {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let denominator = dot(self.normal, ray.direction);
        if denominator.abs() < 1e-6 { 
            return None;
        }
        None
    }
}

#[derive(Deserialize)]
struct PlaneConfig {
    position: Vec3,
    normal: Vec3,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: PlaneConfig = cfg.clone().into_rust().expect("invalid plane config");
    Box::new(Plane::new(config.position, config.normal));
}