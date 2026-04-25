pub mod preprocessor;

use crate::camera::Camera;
use crate::lights::Light;
use crate::materials::Material;
use crate::primitives::Primitive;
use crate::rendering::transform::Transform;

pub type Object = (Primitive, Material, Transform);

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            camera,
        }
    }

    pub fn add_object(&mut self, primitive: Primitive, material: Material, transform: Transform) {
        self.objects.push((primitive, material, transform));
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
}
