use crate::{
    maths::vec3::{Direction, Position},
    rendering::transform::Transform,
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
}
