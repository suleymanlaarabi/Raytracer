use raytracer::maths::vec3::{Vec3, dot};
use raytracer::primitives::Primitive;
use raytracer::rendering::aabb::Aabb;
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;
use serde::Deserialize;

pub struct Plane {
    pub normal: Vec3,
}

impl Plane {
    pub fn new(normal: Vec3) -> Self {
        Plane { normal }
    }
}

impl CanHit for Plane {
    fn aabb(&self, transform: &Transform) -> Aabb {
        let c = transform.translation;
        let r = 1e4_f32;
        Aabb::new(
            Vec3::from_xyz(c.x - r, c.y - r, c.z - r),
            Vec3::from_xyz(c.x + r, c.y + r, c.z + r),
        )
    }

    fn hit(&self, ray: &Ray, transform: &Transform) -> Option<HitRecord> {
        let current_normal = transform.rotate_vec(self.normal);
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
