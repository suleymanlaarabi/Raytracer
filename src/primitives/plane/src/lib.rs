use raytracer::primitives::Primitive;
use serde::Deserialize;

use raytracer::maths::vec3::{Vec3, dot};
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;

pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
}

impl CanHit for Plane {
    fn hit(&self, ray: &Ray, _transform: &Transform) -> Option<HitRecord> {
        let denominator = dot(self.normal, ray.direction);
        if denominator.abs() < 1e-6 { // if denominator equals 0, the ray is perpendicular to the normal
            return None;              // (therefore parallel to the plane), it does not touch it.
        }
        let difference = self.position - ray.position;
        let t = dot(difference, self.normal) / denominator; // determine the distance to the plane using the camera's radius
        if t < 0.001 {
            return None;
        }
        let point = ray.position + t * ray.direction; // find where the impact lies 
        let normal = self.normal;
        Some(HitRecord { t, point, normal})
    }
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Plane { position, normal }
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
    Box::new(Plane::new(config.position, config.normal))
}


