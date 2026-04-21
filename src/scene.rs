use std::io::Write;

use crate::{
    camera::Camera,
    color::Color,
    primitives::sphere::Sphere,
    ray::{CanHit, Ray},
    vec3::{Position, Vec3},
};

#[derive(Default)]
pub struct Scene {
    primitives: Vec<Box<dyn CanHit>>,
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
        let horizontal = Vec3::from_xyz(viewport_width, 0.0, 0.0);
        let vertical = Vec3::from_xyz(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from_xyz(0.0, 0.0, focal_length);

        buffer.clear();
        for y in 0..self.camera.resolution.height {
            for x in 0..self.camera.resolution.width {
                let u = x as f32 / (self.camera.resolution.width - 1) as f32;
                let v = y as f32 / (self.camera.resolution.height - 1) as f32;

                let ray = Ray::new(
                    origin,
                    (lower_left_corner + u * horizontal + v * vertical - origin).normalize(),
                );
                let mut color = Color::BLACK;

                for sphere in &self.primitives {
                    if ray.hit(sphere) {
                        color = Color::RED;
                        break;
                    }
                }
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

    pub fn add_sphere(&mut self, position: Position, radius: f32) {
        self.primitives
            .push(Box::new(Sphere::new(position, radius)));
    }
}
