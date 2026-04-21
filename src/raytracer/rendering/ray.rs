use crate::maths::vec3::{Direction, Position};

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

    pub fn hit(&self, hittable: &dyn CanHit) -> bool {
        hittable.hit(self)
    }
}

pub trait CanHit {
    fn hit(&self, ray: &Ray) -> bool;
}
