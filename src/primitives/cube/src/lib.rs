use raytracer::maths::vec3::Vec3;
use raytracer::primitives::Primitive;
use raytracer::rendering::aabb::Aabb;
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;
use serde::Deserialize;

pub struct Cube {
    pub half_size: Vec3,
}

impl CanHit for Cube {
    fn aabb(&self, transform: &Transform) -> Aabb {
        let h = self.half_size;
        let mut bbox = Aabb::EMPTY;
        for &sx in &[-1.0_f32, 1.0] {
            for &sy in &[-1.0_f32, 1.0] {
                for &sz in &[-1.0_f32, 1.0] {
                    let corner = Vec3::from_xyz(sx * h.x, sy * h.y, sz * h.z);
                    let world = transform.rotate_vec(corner) + transform.translation;
                    bbox = bbox.extend(world);
                }
            }
        }
        bbox
    }

    fn hit(&self, ray: &Ray, transform: &Transform) -> Option<HitRecord> {
        let m = &transform.rot_mat;

        let local_origin = {
            let o = ray.position - transform.translation;
            Vec3::from_xyz(
                m[0][0] * o.x + m[1][0] * o.y + m[2][0] * o.z,
                m[0][1] * o.x + m[1][1] * o.y + m[2][1] * o.z,
                m[0][2] * o.x + m[1][2] * o.y + m[2][2] * o.z,
            )
        };
        let local_dir = Vec3::from_xyz(
            m[0][0] * ray.direction.x + m[1][0] * ray.direction.y + m[2][0] * ray.direction.z,
            m[0][1] * ray.direction.x + m[1][1] * ray.direction.y + m[2][1] * ray.direction.z,
            m[0][2] * ray.direction.x + m[1][2] * ray.direction.y + m[2][2] * ray.direction.z,
        );

        let inv = Vec3::from_xyz(1.0 / local_dir.x, 1.0 / local_dir.y, 1.0 / local_dir.z);

        let t1 = (-self.half_size - local_origin) * inv;
        let t2 = (self.half_size - local_origin) * inv;

        let t_min = Vec3::from_xyz(t1.x.min(t2.x), t1.y.min(t2.y), t1.z.min(t2.z));
        let t_max = Vec3::from_xyz(t1.x.max(t2.x), t1.y.max(t2.y), t1.z.max(t2.z));

        let t_enter = t_min.x.max(t_min.y).max(t_min.z);
        let t_exit = t_max.x.min(t_max.y).min(t_max.z);

        if t_enter > t_exit || t_exit < 0.001 {
            return None;
        }

        let t = if t_enter > 0.001 { t_enter } else { t_exit };

        let local_normal = if t_min.x >= t_min.y && t_min.x >= t_min.z {
            Vec3::from_xyz(if local_dir.x < 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0)
        } else if t_min.y >= t_min.z {
            Vec3::from_xyz(0.0, if local_dir.y < 0.0 { 1.0 } else { -1.0 }, 0.0)
        } else {
            Vec3::from_xyz(0.0, 0.0, if local_dir.z < 0.0 { 1.0 } else { -1.0 })
        };

        Some(HitRecord {
            t,
            point: ray.position + t * ray.direction,
            normal: transform.rotate_vec(local_normal),
        })
    }
}

#[derive(Deserialize)]
struct CubeConfig {
    half_size: Vec3,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: CubeConfig = cfg.clone().into_rust().expect("invalid cube config");
    Box::new(Cube {
        half_size: config.half_size,
    })
}
