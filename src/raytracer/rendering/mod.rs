use std::{
    io::{BufWriter, Write},
    thread,
};

use crate::{
    maths::vec3::Vec3,
    rendering::{color::Color, ray::Ray},
    scene::{Object, Scene},
};

pub mod color;
pub mod ray;
pub mod transform;

pub struct Renderer {
    scene: Scene,
    buffer: Vec<Color>,
}

#[inline]
fn render_x(ray: &mut Ray, objects: &[Object], row: &mut [Color], x: usize) {
    let mut closest = None;
    for (primitive, material, transform) in objects {
        if let Some(hit) = ray.hit(primitive.as_ref(), transform) {
            let is_closer = closest
                .as_ref()
                .is_none_or(|(prev_t, _, _)| hit.t < *prev_t);
            if is_closer {
                closest = Some((hit.t, hit, material.as_ref()));
            }
        }
    }

    row[x] = match closest {
        Some((_, hit, mat)) => mat.shade(&hit),
        None => Color::BLACK,
    };
}

impl Renderer {
    pub fn from_scene(scene: Scene) -> Self {
        Self {
            buffer: Vec::with_capacity(
                (scene.camera.resolution.width * scene.camera.resolution.height) as usize,
            ),
            scene,
        }
    }

    pub fn render(&mut self) {
        let screen_width = self.scene.camera.resolution.width as usize;
        let screen_height = self.scene.camera.resolution.height as usize;

        self.buffer.resize(
            (self.scene.camera.resolution.width * self.scene.camera.resolution.height) as usize,
            Color::BLACK,
        );
        let width = self.scene.camera.resolution.width as f32;
        let height = self.scene.camera.resolution.height as f32;

        let aspect_ratio: f32 = width / height;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = self.scene.camera.fov;

        let origin = self.scene.camera.position;
        let basis = self.scene.camera.basis();

        // multiply by u (0→1) to move the radius from left to right on the viewport
        let horizontal = basis.right * viewport_width;
        // multiplied by v (0→1) to move the radius from bottom to top on the viewport
        let vertical = basis.up * viewport_height;
        // starting point of the interpolation: the radius (u=0, v=0) starts from here
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 + basis.forward * focal_length;

        let objects: &[Object] = &self.scene.objects;

        let threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let rows_per_chunk = screen_height.div_ceil(threads);

        thread::scope(|s| {
            for (chunk_idx, rows) in self
                .buffer
                .chunks_mut(rows_per_chunk * screen_width)
                .enumerate()
            {
                let start_y = chunk_idx * rows_per_chunk;

                s.spawn(move || {
                    for (local_y, row) in rows.chunks_mut(screen_width).enumerate() {
                        let y = start_y + local_y;

                        let v = y as f32 / (screen_height - 1) as f32;
                        let mut ray = Ray::new(origin, Vec3::ZERO);

                        for x in 0..screen_width {
                            let u = x as f32 / (screen_width - 1) as f32;

                            ray.direction = (lower_left_corner + u * horizontal + v * vertical
                                - origin)
                                .normalize();

                            render_x(&mut ray, objects, row, x);
                        }
                    }
                });
            }
        });
    }

    pub fn render_to_file(&mut self, path: &str) {
        self.render();

        if !path.ends_with(".ppm") {
            panic!("Only .ppm format is supported");
        }

        let file = std::fs::File::create(path).expect("Failed to create file");
        let header = format!(
            "P6\n{} {}\n255\n",
            self.scene.camera.resolution.width, self.scene.camera.resolution.height
        );

        let mut file = BufWriter::new(file);

        file.write_all(header.as_bytes())
            .expect("Failed to write header");

        let bytes = unsafe {
            std::slice::from_raw_parts(self.buffer.as_ptr() as *const u8, self.buffer.len() * 3)
        };
        file.write_all(bytes).expect("Failed to write pixel data");

        file.flush().expect("Failed to flush file");
    }
}
