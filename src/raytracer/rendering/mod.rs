use std::{
    io::{BufWriter, Write},
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

use crate::{
    lights::{Light, LightSample},
    maths::vec3::Vec3,
    rendering::{bvh::Bvh, color::Color, ray::Ray},
    scene::{Object, Scene},
};

const TILE_SIZE: usize = 16;

pub mod aabb;
pub mod bvh;
pub mod color;
pub mod ray;
pub mod transform;

pub struct Renderer {
    pub scene: Scene,
    bvh: Bvh,
    pub buffer: Vec<Color>,
}

#[inline(always)]
fn shade_ray(
    ray: &Ray,
    inv_dir: Vec3,
    bvh: &Bvh,
    objects: &[Object],
    lights: &[Light],
    light_buf: &mut Vec<LightSample>,
) -> Color {
    match bvh.traverse(ray, inv_dir, objects) {
        Some((hit, mat)) => {
            light_buf.clear();
            for light in lights {
                let s = light.sample(hit.point);
                let origin = hit.point + s.direction * 0.0001;
                let shadow_ray = Ray::new(origin, s.direction);
                let shadow_inv = Vec3::from_xyz(
                    1.0 / s.direction.x,
                    1.0 / s.direction.y,
                    1.0 / s.direction.z,
                );
                if !bvh.hit_any(&shadow_ray, shadow_inv, s.distance, objects) {
                    light_buf.push(s);
                }
            }
            mat.shade(&hit, light_buf)
        }
        None => Color::BLACK,
    }
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

        self.buffer
            .resize(screen_width * screen_height, Color::BLACK);

        let aspect_ratio = screen_width as f32 / screen_height as f32;
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

        let tiles_x = screen_width.div_ceil(TILE_SIZE);
        let tiles_y = screen_height.div_ceil(TILE_SIZE);
        let total_tiles = tiles_x * tiles_y;
        let tile_counter = AtomicUsize::new(0);

        let buf_addr = self.buffer.as_mut_ptr() as usize;

        thread::scope(|s| {
            for _ in 0..threads {
                let tile_counter = &tile_counter;
                s.spawn(move || {
                    let buf = buf_addr as *mut Color;
                    let mut light_buf: Vec<LightSample> = Vec::with_capacity(lights.len());
                    loop {
                        let tile_idx = tile_counter.fetch_add(1, Ordering::Relaxed);
                        if tile_idx >= total_tiles {
                            break;
                        }
                        let tile_x0 = (tile_idx % tiles_x) * TILE_SIZE;
                        let tile_y0 = (tile_idx / tiles_x) * TILE_SIZE;
                        let x_end = (tile_x0 + TILE_SIZE).min(screen_width);
                        let y_end = (tile_y0 + TILE_SIZE).min(screen_height);

                        for y in tile_y0..y_end {
                            let v = 1.0 - y as f32 / (screen_height - 1) as f32;
                            let v_contrib = lower_left_corner + v * vertical;
                            for x in tile_x0..x_end {
                                let u = x as f32 / (screen_width - 1) as f32;
                                let direction = (v_contrib + u * horizontal - origin).normalize();
                                let inv_dir = Vec3::from_xyz(
                                    1.0 / direction.x,
                                    1.0 / direction.y,
                                    1.0 / direction.z,
                                );
                                let ray = Ray::new(origin, direction);
                                let color =
                                    shade_ray(&ray, inv_dir, bvh, objects, lights, &mut light_buf);

                                unsafe {
                                    *buf.add(y * screen_width + x) = color;
                                }
                            }
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
