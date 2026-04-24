use raytracer::primitives::Primitive;
use serde::Deserialize;

use raytracer::maths::vec3::{Vec3, dot};
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;

pub struct Plane {
    pub normal: Vec3,
}

impl Plane {
    pub fn new(normal: Vec3) -> Self {
        Plane { normal }
    }
}

impl CanHit for Plane {
    fn hit(&self, ray: &Ray, transform: &Transform) -> Option<HitRecord> {
        let current_normal = self.normal.rotate(transform.rotation);
        let denominator = dot(current_normal, ray.direction);
        if denominator.abs() < 1e-6 {
            // if denominator equals 0, the ray is perpendicular to the normal
            return None; // (therefore parallel to the plane), it does not touch it.
        }
        let difference = transform.translation - ray.position;
        let t = dot(difference, current_normal) / denominator; // determine the distance to the plane
        if t < 0.001 {
            return None;
        }

        let point = ray.position + t * ray.direction; // find where the impact lies 

        Some(HitRecord {
            t,
            point,
            normal: current_normal,
        })
    }
}

#[derive(Deserialize)]
struct PlaneConfig {
    normal: Vec3,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: PlaneConfig = cfg.clone().into_rust().expect("invalid plane config");
    Box::new(Plane::new(config.normal))
}
