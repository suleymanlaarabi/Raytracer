use crate::rendering::ray::CanHit;
pub type Primitive = Box<dyn CanHit>;
