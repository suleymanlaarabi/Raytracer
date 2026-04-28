use std::{
    io::{BufWriter, Write},
    thread,
};

use crate::{
    lights::{Light, LightSample},
    maths::vec3::Vec3,
    rendering::{bvh::Bvh, color::Color, ray::Ray},
    scene::{Object, Scene},
};

pub mod aabb;
pub mod bvh;
pub mod color;
pub mod ray;
pub mod transform;

pub struct Renderer {
    scene: Scene,
    bvh: Bvh,
    buffer: Vec<Color>,
}

#[inline]
fn render_x(
    ray: &Ray,
    inv_dir: Vec3,
    bvh: &Bvh,
    objects: &[Object],
    lights: &[Light],
    row: &mut [Color],
    x: usize,
    light_buf: &mut Vec<LightSample>,
) {
    row[x] = match bvh.traverse(ray, inv_dir, objects) {
        Some((hit, mat)) => {
            light_buf.clear();
            light_buf.extend(lights.iter().map(|l| l.sample(hit.point)).filter(|s| {
                let origin = hit.point + s.direction * 0.0001;
                let shadow_ray = Ray::new(origin, s.direction);
                let shadow_inv = Vec3::from_xyz(
                    1.0 / s.direction.x,
                    1.0 / s.direction.y,
                    1.0 / s.direction.z,
                );
                !bvh.hit_any(&shadow_ray, shadow_inv, s.distance, objects)
            }));
            mat.shade(&hit, light_buf)
        }
        None => Color::BLACK,
    };
}

impl Renderer {
    pub fn from_scene(scene: Scene) -> Self {
        let bvh = Bvh::build(&scene.objects);
        Self {
            buffer: Vec::with_capacity(
                (scene.camera.resolution.width * scene.camera.resolution.height) as usize,
            ),
            bvh,
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

        let aspect_ratio =
            self.scene.camera.resolution.width as f32 / self.scene.camera.resolution.height as f32;
        let viewport_height = 2.0_f32;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = self.scene.camera.fov;

        let origin = self.scene.camera.position;
        let basis = self.scene.camera.basis();
        let horizontal = basis.right * viewport_width;
        let vertical = basis.up * viewport_height;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 + basis.forward * focal_length;

        let bvh = &self.bvh;
        let objects: &[Object] = &self.scene.objects;
        let lights: &[Light] = &self.scene.lights;

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
                    let mut light_buf: Vec<LightSample> = Vec::with_capacity(lights.len());
                    for (local_y, row) in rows.chunks_mut(screen_width).enumerate() {
                        let y = start_y + local_y;
                        let v = 1.0 - y as f32 / (screen_height - 1) as f32;
                        let v_contrib = lower_left_corner + v * vertical;

                        for x in 0..screen_width {
                            let u = x as f32 / (screen_width - 1) as f32;
                            let direction = (v_contrib + u * horizontal - origin).normalize();
                            let inv_dir = Vec3::from_xyz(
                                1.0 / direction.x,
                                1.0 / direction.y,
                                1.0 / direction.z,
                            );
                            let ray = Ray::new(origin, direction);
                            render_x(&ray, inv_dir, bvh, objects, lights, row, x, &mut light_buf);
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
