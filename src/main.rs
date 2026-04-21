use serde::Deserialize;

use crate::camera::Camera;
use crate::errors::errors::RaytracerError;
use crate::errors::errors::RaytracerError::IncorrectArguments;
use crate::{scene::Scene, vec3::Position};
use std::env::args;
use std::error::Error;

mod camera;
mod color;
mod errors;
mod primitives;
mod ray;
mod scene;
mod vec3;

fn get_config_file() -> Result<String, RaytracerError> {
    let mut args = args().collect::<Vec<String>>();

    if args.len() != 2 {
        return Err(IncorrectArguments);
    }

    Ok(args.remove(1))
}

#[derive(Deserialize)]
struct SceneDesc {
    primitives: Vec<ron::Value>,
    lights: Vec<ron::Value>,
    camera: Camera,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = get_config_file()?;

    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_file))
        .build()?;

    let parsed = config.try_deserialize::<SceneDesc>().unwrap();

    println!("{:?}", parsed.camera);

    // let mut scene = Scene::default();

    // scene.add_sphere(Position::from_xyz(1., 0.0, -3.0), 1.0);

    // scene.render_to_file("image.ppm");

    Ok(())
}
