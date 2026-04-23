use std::{fs, str::FromStr};

use raytracer::{
    maths::{
        rotation::{rotate, rotate_inverse},
        vec3::{Vec3, cross, dot},
    },
    primitives::Primitive,
    rendering::{
        ray::{CanHit, HitRecord, Ray},
        transform::Transform,
    },
};
use serde::Deserialize;

const EPSILON: f32 = 1e-7;
const T_MIN: f32 = 1e-4;

#[derive(Copy, Clone)]
struct Tri(Vec3, Vec3, Vec3, Vec3);

pub struct Object(Vec<Tri>);

impl CanHit for Object {
    fn hit(&self, ray: &Ray, transform: &Transform) -> Option<HitRecord> {
        let local_pos = rotate_inverse(ray.position - transform.translation, transform.rotation);
        let local_dir = rotate_inverse(ray.direction, transform.rotation);
        let local_ray = Ray::new(local_pos, local_dir);

        let (mut best, mut t_max) = (None, f32::MAX);
        for tri in &self.0 {
            if let Some(hit) = hit_tri(&local_ray, tri, t_max) {
                t_max = hit.t;
                best = Some(hit);
            }
        }

        best.map(|hit| HitRecord {
            t: hit.t,
            point: rotate(hit.point, transform.rotation) + transform.translation,
            normal: rotate(hit.normal, transform.rotation),
        })
    }
}

fn hit_tri(ray: &Ray, Tri(v0, e1, e2, n): &Tri, t_max: f32) -> Option<HitRecord> {
    let p = cross(ray.direction, *e2);
    let det = dot(*e1, p);
    if det.abs() < EPSILON {
        return None;
    }

    let inv_det = det.recip();
    let tvec = ray.position - *v0;
    let u = dot(tvec, p) * inv_det;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = cross(tvec, *e1);
    let v = dot(ray.direction, q) * inv_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = dot(*e2, q) * inv_det;
    (T_MIN..=t_max).contains(&t).then(|| HitRecord {
        t,
        point: ray.position + ray.direction * t,
        normal: *n,
    })
}

impl Object {
    pub fn new(path: &str) -> Self {
        let ObjModel {
            vertices,
            triangles,
        } = fs::read_to_string(path)
            .expect("unable to read .obj")
            .parse::<ObjModel>()
            .expect("invalid obj file");

        Self(
            triangles
                .into_iter()
                .map(|[a, b, c]| {
                    let v0 = vertices[a];
                    let e1 = vertices[b] - v0;
                    let e2 = vertices[c] - v0;
                    Tri(v0, e1, e2, cross(e1, e2).normalize())
                })
                .collect(),
        )
    }
}

#[derive(Deserialize)]
struct ObjectConfig {
    path: String,
}

#[derive(Default)]
struct ObjModel {
    vertices: Vec<Vec3>,
    triangles: Vec<[usize; 3]>,
}

fn face_idx(s: &str) -> Option<usize> {
    Some(
        s.split('/')
            .next()?
            .parse::<usize>()
            .ok()?
            .saturating_sub(1),
    )
}

impl FromStr for ObjModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut obj = Self::default();

        for line in s.lines() {
            let mut it = line.split_whitespace();

            match it.next() {
                Some("v") => {
                    let mut next = || it.next().ok_or(())?.parse::<f32>().map_err(|_| ());
                    obj.vertices.push(Vec3::from_xyz(next()?, next()?, next()?));
                }
                Some("f") => {
                    let Some(a) = it.next().and_then(face_idx) else {
                        continue;
                    };
                    let Some(mut b) = it.next().and_then(face_idx) else {
                        continue;
                    };

                    for c in it.filter_map(face_idx) {
                        obj.triangles.push([a, b, c]);
                        b = c;
                    }
                }
                _ => {}
            }
        }

        Ok(obj)
    }
}

#[unsafe(no_mangle)]
pub fn create(cfg: &ron::Value) -> Primitive {
    let config: ObjectConfig = cfg
        .clone()
        .into_rust()
        .expect("object configuration invalid");

    Box::new(Object::new(&config.path))
}
