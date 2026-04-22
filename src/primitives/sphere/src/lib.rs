use raytracer::primitives::Primitive;
use serde::Deserialize;

use raytracer::maths::vec3::{Position, Vec3, dot};
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
}

impl CanHit for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.position - self.position;
        let a = dot(ray.direction, ray.direction);
        let half_b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let t1 = (-half_b - sqrt_d) / a;
        let t2 = (-half_b + sqrt_d) / a;
        let t = if t1 > 0.0 {
            t1
        } else if t2 > 0.0 {
            t2
        } else {
            return None;
        };
        let point = ray.position + t * ray.direction;
        let normal = (point - self.position) / self.radius;
        Some(HitRecord { t, point, normal })
    }
}

impl Sphere {
    pub fn new(position: Position, radius: f32) -> Self {
        Sphere { position, radius }
    }
}

#[derive(Deserialize)]
struct SphereConfig {
    position: Position,
    radius: f32,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: SphereConfig = cfg.clone().into_rust().expect("invalid sphere config");
    Box::new(Sphere::new(config.position, config.radius))
}
