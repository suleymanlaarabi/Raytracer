use std::io::Write;

use crate::camera::Camera;
use crate::materials::Material;
use crate::primitives::Primitive;
use crate::rendering::color::Color;
use crate::rendering::render_scene;
use crate::rendering::transform::Transform;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<(Primitive, Material, Transform)>,
    pub camera: Camera,
}

impl Scene {
    pub fn render(&self, buffer: &mut Vec<Color>) {
        render_scene(self, buffer);
    }

    pub fn render_to_file(&self, path: &str) {
        let mut buffer = Vec::new();
        self.render(&mut buffer);

        if !path.ends_with(".ppm") {
            panic!("Only .ppm format is supported");
        }

        let mut file = std::fs::File::create(path).expect("Failed to create file");
        let header = format!(
            "P3\n{} {}\n255\n",
            self.camera.resolution.width, self.camera.resolution.height
        );
        file.write_all(header.as_bytes())
            .expect("Failed to write header");

        for color in buffer {
            let pixel = format!("{} {} {}\n", color.r, color.g, color.b);
            file.write_all(pixel.as_bytes())
                .expect("Failed to write pixel data");
        }
    }

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
