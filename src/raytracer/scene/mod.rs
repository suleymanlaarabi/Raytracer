use std::io::Write;

use crate::camera::Camera;
use crate::materials::Material;
use crate::primitives::Primitive;
use crate::rendering::color::Color;
use crate::rendering::ray::Ray;

#[derive(Default)]
pub struct Scene {
    objects: Vec<(Primitive, Material)>,
    camera: Camera,
}

impl Scene {
    pub fn render(&self, buffer: &mut Vec<Color>) {
        buffer.reserve((self.camera.resolution.width * self.camera.resolution.height) as usize);
        let width = self.camera.resolution.width as f32;
        let height = self.camera.resolution.height as f32;

        let aspect_ratio: f32 = width / height;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = self.camera.fov;

        let origin = self.camera.position;
        let basis = self.camera.basis();

        // multiply by u (0→1) to move the radius from left to right on the viewport
        let horizontal = basis.right * viewport_width;
        // multiplied by v (0→1) to move the radius from bottom to top on the viewport
        let vertical = basis.up * viewport_height;
        // starting point of the interpolation: the radius (u=0, v=0) starts from here
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 + basis.forward * focal_length;

        buffer.clear();
        for y in 0..self.camera.resolution.height {
            for x in 0..self.camera.resolution.width {
                let u = x as f32 / (self.camera.resolution.width - 1) as f32;
                let v = y as f32 / (self.camera.resolution.height - 1) as f32;

                let ray = Ray::new(
                    origin,
                    (lower_left_corner + u * horizontal + v * vertical - origin).normalize(),
                );

                let mut closest = None;
                for (primitive, material) in &self.objects {
                    // TODO: actually need to check what object is the most closest
                    // (en fr excuse pavel) il faudrait qu'on sorte les objet du plus proche aux plus loin de la camera
                    // comme ca on pourrait sarreter aux premier hit pour chaque pixel
                    if let Some(hit) = ray.hit(primitive.as_ref()) {
                        let is_closer = closest
                            .as_ref()
                            .is_none_or(|(prev_t, _, _)| hit.t < *prev_t);
                        if is_closer {
                            closest = Some((hit.t, hit, material.as_ref()));
                        }
                    }
                }

                let color = match closest {
                    Some((_, hit, mat)) => mat.shade(&hit),
                    None => Color::BLACK,
                };
                buffer.push(color);
            }
        }
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

    pub fn add_object(&mut self, primitive: Primitive, material: Material) {
        self.objects.push((primitive, material));
    }
}
