use raytracer::primitives::Primitive;
use serde::Deserialize;

use raytracer::maths::vec3::{Vec3, dot};
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;

pub struct Cone {
    pub angle_rad: f32,
    pub cos2: f32,
}

impl Cone {
    pub fn new(angle_deg: f32) -> Self {
        let angle_rad = angle_deg.to_radians(); // required to calculate in radians for trigonometry in radians
        let cos2 = angle_rad.cos() * angle_rad.cos();
        Cone { angle_rad, cos2 }
    }
}

impl CanHit for Cone {
    fn hit(&self, ray: &Ray, transform: &Transform) -> Option<HitRecord> {
        let base_axis = Vec3::from_xyz(0.0, 1.0, 0.0);
        let cone_axis = base_axis.rotate(transform.rotation).normalize(); // steering axis

        let apex = transform.translation; // tip of the cone
        let apex_to_ray = ray.position - apex;

        let dir_dot_axis = dot(ray.direction, cone_axis);
        let apex_to_ray_dot_axis = dot(apex_to_ray, cone_axis);
        let apex_to_ray_dot_dir = dot(apex_to_ray, ray.direction);
        let dir_sq_len = dot(ray.direction, ray.direction);
        let apex_to_ray_sq_len = dot(apex_to_ray, apex_to_ray);

        let a = dir_dot_axis * dir_dot_axis - dir_sq_len * self.cos2; // at² + bt + c = 0
        let b = 2.0 * (apex_to_ray_dot_axis * dir_dot_axis - apex_to_ray_dot_dir * self.cos2); // formula for the intersection between a line and an infinite cone 
        let c_eq = apex_to_ray_dot_axis * apex_to_ray_dot_axis - apex_to_ray_sq_len * self.cos2; // we find the distance at which the ray hits the surface.

        let delta = b * b - 4.0 * a * c_eq;
        if delta < 0.0 {
            return None;
        }

        let sqrt_delta = delta.sqrt(); // find the distance of the impact
        let t1 = (-b - sqrt_delta) / (2.0 * a);
        let t2 = (-b + sqrt_delta) / (2.0 * a);

        let t = if t1 > 0.001 && t2 > 0.001 {
            t1.min(t2)
        } else if t1 > 0.001 {
            t1
        } else if t2 > 0.001 {
            t2
        } else {
            return None;
        };

        let point = ray.position + ray.direction * t; // Calculation of the 3D point and the normal
        let apex_to_impact = point - apex;
        let projection_length = dot(apex_to_impact, cone_axis);
        let raw_normal = apex_to_impact * self.cos2 - cone_axis * projection_length;
        let normal = raw_normal.normalize();

        Some(HitRecord { t, point, normal })
    }
}

#[derive(Deserialize)]
struct ConeConfig {
    angle: f32,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: ConeConfig = cfg.clone().into_rust().expect("invalid cone config");
    Box::new(Cone::new(config.angle))
}
