use crate::vec3::{Direction, Position};

pub struct Ray {
    pub position: Position,
    pub direction: Direction,
}

impl Ray {
    pub const fn new(position: Position, direction: Direction) -> Self {
        return Self {
            position,
            direction,
        };
    }

    pub fn hit(&self, hittable: &Box<dyn CanHit>) -> bool {
        return hittable.hit(self);
    }
}

pub trait CanHit {
    fn hit(&self, ray: &Ray) -> bool;
}
