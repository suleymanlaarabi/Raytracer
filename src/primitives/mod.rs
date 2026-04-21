use crate::ray::CanHit;

pub mod sphere;

type Primitive = Box<dyn CanHit>;

unsafe extern "Rust" {
    pub fn create(cfg: &ron::Value) -> Primitive;
}

struct PrimitiveFactory {}
