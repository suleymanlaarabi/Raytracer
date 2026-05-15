use raytracer::maths::vec3::Vec3;
use raytracer::primitives::Primitive;
use raytracer::rendering::aabb::Aabb;
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;
use serde::Deserialize;

pub struct Torus {
    pub major_radius: f32,
    pub minor_radius: f32,
}

impl Torus {
    pub fn new(major_radius: f32, minor_radius: f32) -> Self {
        Torus {
            major_radius,
            minor_radius,
        }
    }

    #[inline]
    fn sdf(&self, p: Vec3) -> f32 {
        let qx = (p.x * p.x + p.z * p.z).sqrt() - self.major_radius;
        let qy = p.y;
        (qx * qx + qy * qy).sqrt() - self.minor_radius
    }

    fn normal_local(&self, p: Vec3) -> Option<Vec3> {
        let r_major2 = self.major_radius * self.major_radius;
        let r_minor2 = self.minor_radius * self.minor_radius;
        let sum = p.x * p.x + p.y * p.y + p.z * p.z;
        let n = Vec3::from_xyz(
            p.x * (sum - r_major2 - r_minor2),
            p.y * (sum + r_major2 - r_minor2),
            p.z * (sum - r_major2 - r_minor2),
        );
        let len2 = n.x * n.x + n.y * n.y + n.z * n.z;
        (len2 > 1e-8).then(|| n.normalize())
    }
}

impl CanHit for Torus {
    fn aabb(&self, transform: &Transform) -> Aabb {
        let c = transform.translation;
        let r = self.major_radius + self.minor_radius;
        Aabb::new(
            Vec3::from_xyz(c.x - r, c.y - r, c.z - r),
            Vec3::from_xyz(c.x + r, c.y + r, c.z + r),
        )
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

        let mut t = 0.001;
        for _ in 0..1024 {
            if t > 300.0 {
                return None;
            }

            let point_local = local_origin + t * local_dir;
            let d = self.sdf(point_local);

            if d.abs() <= 3e-4 {
                let normal_local = self.normal_local(point_local)?;
                let normal = transform.rotate_vec(normal_local).normalize();
                let point = ray.position + t * ray.direction + normal * 2e-3;
                return Some(HitRecord { t, point, normal });
            }

            t += d.abs().max(5e-4);
        }

        None
    }
}

#[derive(Deserialize)]
struct TorusConfig {
    major_radius: f32,
    minor_radius: f32,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: TorusConfig = cfg.clone().into_rust().expect("invalid torus config");
    Box::new(Torus::new(config.major_radius, config.minor_radius))
}
