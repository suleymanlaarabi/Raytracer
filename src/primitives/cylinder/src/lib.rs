use raytracer::maths::vec3::{Vec3, dot};
use raytracer::primitives::Primitive;
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;
use serde::Deserialize;

pub struct Cylinder {
    pub radius: f32,
    pub height: f32,
}

impl CanHit for Cylinder {
    fn hit(&self, ray: &Ray, transform: &Transform) -> Option<HitRecord> {
        let axis = Vec3::from_xyz(0.0, 1.0, 0.0)
            .rotate(transform.rotation)
            .normalize();
        let half_h = self.height / 2.0;
        let oc = ray.position - transform.translation;
        let d_perp = ray.direction - dot(ray.direction, axis) * axis;
        let oc_perp = oc - dot(oc, axis) * axis;
        let r2 = self.radius * self.radius;

        let mut best: Option<(f32, Vec3)> = None;

        let a = dot(d_perp, d_perp);
        if a > 1e-6 {
            let hb = dot(oc_perp, d_perp);
            let disc = hb * hb - a * (dot(oc_perp, oc_perp) - r2);
            if disc >= 0.0 {
                let sd = disc.sqrt();
                for t in [(-hb - sd) / a, (-hb + sd) / a] {
                    if t > 0.001 {
                        let p = ray.position + t * ray.direction;
                        let y = dot(p - transform.translation, axis);
                        if y.abs() <= half_h {
                            best =
                                Some((t, ((p - transform.translation) - y * axis) / self.radius));
                            break;
                        }
                    }
                }
            }
        }

        for sign in [1.0_f32, -1.0] {
            let n = axis * sign;
            let center = transform.translation + sign * half_h * axis;
            let denom = dot(ray.direction, n);
            if denom.abs() > 1e-6 {
                let t = dot(center - ray.position, n) / denom;
                if t > 0.001 && best.is_none_or(|(bt, _)| t < bt) {
                    let diff = ray.position + t * ray.direction - center;
                    let dp = diff - dot(diff, axis) * axis;
                    if dot(dp, dp) <= r2 {
                        best = Some((t, n));
                    }
                }
            }
        }

        best.map(|(t, normal)| HitRecord {
            t,
            point: ray.position + t * ray.direction,
            normal,
        })
    }
}

#[derive(Deserialize)]
struct CylinderConfig {
    radius: f32,
    height: f32,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: CylinderConfig = cfg.clone().into_rust().expect("invalid cylinder config");
    Box::new(Cylinder {
        radius: config.radius,
        height: config.height,
    })
}
