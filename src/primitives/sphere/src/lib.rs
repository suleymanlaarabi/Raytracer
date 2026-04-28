use raytracer::maths::vec3::{Vec3, dot};
use raytracer::primitives::Primitive;
use raytracer::rendering::aabb::Aabb;
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;
use serde::Deserialize;

pub struct Sphere {
    pub radius: f32,
    pub inv_radius: f32,
}

impl CanHit for Sphere {
    fn aabb(&self, transform: &Transform) -> Aabb {
        let c = transform.translation;
        Aabb::new(
            Vec3::from_xyz(c.x - self.radius, c.y - self.radius, c.z - self.radius),
            Vec3::from_xyz(c.x + self.radius, c.y + self.radius, c.z + self.radius),
        )
    }

    fn hit(&self, ray: &Ray, transform: &Transform) -> Option<HitRecord> {
        let oc = ray.position - transform.translation;
        let half_b = dot(oc, ray.direction);
        let disc = half_b * half_b - (dot(oc, oc) - self.radius * self.radius);
        if disc < 0.0 {
            return None;
        }
        let sqrt_d = disc.sqrt();
        let t1 = -half_b - sqrt_d;
        let t2 = -half_b + sqrt_d;
        let t = if t1 > 0.001 {
            t1
        } else if t2 > 0.001 {
            t2
        } else {
            return None;
        };
        let point = ray.position + t * ray.direction;
        Some(HitRecord {
            t,
            point,
            normal: (point - transform.translation) * self.inv_radius,
        })
    }
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere {
            radius,
            inv_radius: 1.0 / radius,
        }
    }
}

#[derive(Deserialize)]
struct SphereConfig {
    radius: f32,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: SphereConfig = cfg.clone().into_rust().expect("invalid sphere config");
    Box::new(Sphere::new(config.radius))
}
