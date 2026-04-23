use crate::{
    rendering::{color::Color, ray::Ray},
    scene::Scene,
};

pub mod color;
pub mod ray;
pub mod transform;

pub fn render_scene(scene: &Scene, buffer: &mut Vec<Color>) {
    buffer.reserve((scene.camera.resolution.width * scene.camera.resolution.height) as usize);
    let width = scene.camera.resolution.width as f32;
    let height = scene.camera.resolution.height as f32;

    let aspect_ratio: f32 = width / height;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = scene.camera.fov;

    let origin = scene.camera.position;
    let basis = scene.camera.basis();

    // multiply by u (0→1) to move the radius from left to right on the viewport
    let horizontal = basis.right * viewport_width;
    // multiplied by v (0→1) to move the radius from bottom to top on the viewport
    let vertical = basis.up * viewport_height;
    // starting point of the interpolation: the radius (u=0, v=0) starts from here
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 + basis.forward * focal_length;

    buffer.clear();
    for y in 0..scene.camera.resolution.height {
        for x in 0..scene.camera.resolution.width {
            let u = x as f32 / (scene.camera.resolution.width - 1) as f32;
            let v = y as f32 / (scene.camera.resolution.height - 1) as f32;

            let ray = Ray::new(
                origin,
                (lower_left_corner + u * horizontal + v * vertical - origin).normalize(),
            );

            let mut closest = None;
            for (primitive, material, transform) in &scene.objects {
                if let Some(hit) = ray.hit(primitive.as_ref(), transform) {
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
