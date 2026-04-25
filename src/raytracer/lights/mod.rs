use crate::maths::vec3::{Direction, Position};
use crate::rendering::color::ColorF;

pub struct LightSample {
    pub direction: Direction,
    pub distance: f32,
    pub intensity: ColorF,
}

pub trait CanLight {
    fn sample(&self, point: Position) -> LightSample;
}

pub type Light = Box<dyn CanLight + Send + Sync>;
