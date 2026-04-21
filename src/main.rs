use crate::{scene::Scene, vec3::Position};

mod camera;
mod color;
mod primitives;
mod ray;
mod scene;
mod vec3;

fn main() {
    let mut scene = Scene::default();

    scene.add_sphere(Position::from_xyz(1., 0.0, -3.0), 1.0);

    scene.render_to_file("image.ppm");
}
