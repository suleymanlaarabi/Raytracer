use crate::{
    maths::vec3::{Direction, Position, Vec3},
    rendering::{aabb::Aabb, transform::Transform},
};

pub struct HitRecord {
    pub t: f32,
    pub point: Position,
    pub normal: Direction,
}

pub struct Ray {
    pub position: Position,
    pub direction: Direction,
}

impl Ray {
    pub const fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn hit(&self, hittable: &dyn CanHit, transform: &Transform) -> Option<HitRecord> {
        hittable.hit(self, transform)
    }
}

pub trait CanHit {
    fn hit(&self, ray: &Ray, transform: &Transform) -> Option<HitRecord>;

    fn aabb(&self, transform: &Transform) -> Aabb {
        let c = transform.translation;
        let r = 1e4_f32;
        Aabb::new(
            Vec3::from_xyz(c.x - r, c.y - r, c.z - r),
            Vec3::from_xyz(c.x + r, c.y + r, c.z + r),
        )
    }
}
