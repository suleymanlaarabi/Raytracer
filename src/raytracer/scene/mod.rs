pub mod preprocessor;

use crate::camera::Camera;
use crate::materials::Material;
use crate::primitives::Primitive;
use crate::rendering::transform::Transform;

pub type Object = (Primitive, Material, Transform);

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            objects: Vec::new(),
            camera,
        }
    }

    pub fn add_object(&mut self, primitive: Primitive, material: Material, transform: Transform) {
        self.objects.push((primitive, material, transform));
    }
}
