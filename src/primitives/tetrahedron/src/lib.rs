use raytracer::maths::vec3::Vec3;
use raytracer::primitives::Primitive;
use raytracer::rendering::aabb::Aabb;
use raytracer::rendering::ray::{CanHit, HitRecord, Ray};
use raytracer::rendering::transform::Transform;
use serde::Deserialize;

pub struct Tetrahedron {
    pub size: f32,
}

impl Tetrahedron {
    pub fn new(size: f32) -> Self {
        Tetrahedron { size }
    }

    fn get_vertices(&self) -> [Vec3; 4] {
        let s = self.size;
        [
            Vec3::from_xyz(1.0, 1.0, 1.0) * s,
            Vec3::from_xyz(1.0, -1.0, -1.0) * s,
            Vec3::from_xyz(-1.0, 1.0, -1.0) * s,
            Vec3::from_xyz(-1.0, -1.0, 1.0) * s,
        ]
    }

    fn get_faces(&self) -> [(usize, usize, usize); 4] {
        [(0, 1, 2), (0, 2, 3), (0, 3, 1), (1, 3, 2)]
    }
}

impl CanHit for Tetrahedron {
    fn aabb(&self, transform: &Transform) -> Aabb {
        let bound = self.size * 1.5;
        let c = transform.translation;
        Aabb::new(
            Vec3::from_xyz(c.x - bound, c.y - bound, c.z - bound),
            Vec3::from_xyz(c.x + bound, c.y + bound, c.z + bound),
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

        let vertices = self.get_vertices();
        let faces = self.get_faces();

        let mut best_hit: Option<HitRecord> = None;
        let mut best_t = f32::INFINITY;

        for (i0, i1, i2) in faces.iter() {
            let v0 = vertices[*i0];
            let v1 = vertices[*i1];
            let v2 = vertices[*i2];

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let h = cross(local_dir, edge2);
            let a = dot(edge1, h);

            if a.abs() < 1e-6 {
                continue;
            }

            let f = 1.0 / a;
            let s = local_origin - v0;
            let u = f * dot(s, h);

            if !(0.0..=1.0).contains(&u) {
                continue;
            }

            let q = cross(s, edge1);
            let v = f * dot(local_dir, q);

            if v < 0.0 || u + v > 1.0 {
                continue;
            }

            let t = f * dot(edge2, q);

            if t > 0.001 && t < best_t {
                best_t = t;

                let normal_local = cross(edge1, edge2).normalize();

                let dot_product = dot(normal_local, local_dir);
                let normal_local = if dot_product > 0.0 {
                    -normal_local
                } else {
                    normal_local
                };

                let normal = Vec3::from_xyz(
                    m[0][0] * normal_local.x + m[0][1] * normal_local.y + m[0][2] * normal_local.z,
                    m[1][0] * normal_local.x + m[1][1] * normal_local.y + m[1][2] * normal_local.z,
                    m[2][0] * normal_local.x + m[2][1] * normal_local.y + m[2][2] * normal_local.z,
                );

                let point = ray.position + t * ray.direction;
                best_hit = Some(HitRecord { t, point, normal });
            }
        }

        best_hit
    }
}

fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::from_xyz(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

#[derive(Deserialize)]
struct TetrahedronConfig {
    size: f32,
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: TetrahedronConfig = cfg.clone().into_rust().expect("invalid tetrahedron config");
    Box::new(Tetrahedron::new(config.size))
}
