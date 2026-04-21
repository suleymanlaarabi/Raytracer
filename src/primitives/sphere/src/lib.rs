use serde::Deserialize;

use raytracer::maths::vec3::{Position, Vec3, dot};
use raytracer::rendering::ray::{CanHit, Ray};

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
}

impl CanHit for Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.position - self.position;
        let a = dot(ray.direction, ray.direction);
        let half_b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();
        let t1 = (-half_b - sqrt_d) / a;
        let t2 = (-half_b + sqrt_d) / a;
        t1 > 0.0 || t2 > 0.0
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
pub fn create(cfg: &ron::Value) -> Box<dyn CanHit> {
    let config: SphereConfig = cfg.clone().into_rust().expect("invalid sphere config");
    println!("Hello non");
    Box::new(Sphere::new(config.position, config.radius))
}
