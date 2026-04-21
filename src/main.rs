use std::env::args;
use std::error::Error;
use crate::{scene::Scene, vec3::Position};
use crate::errors::errors::RaytracerError;
use crate::errors::errors::RaytracerError::IncorrectArguments;


mod camera;
mod color;
mod primitives;
mod ray;
mod scene;
mod vec3;
mod errors;

fn get_config_file() -> Result<String, RaytracerError> {
    let mut args = args().collect::<Vec<String>>();

    if args.len() != 2 {
        return Err(IncorrectArguments)
    }

    Ok(args.remove(1))
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = get_config_file()?;

    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_file))
        .build()?;

    let mut scene = Scene::default();

    scene.add_sphere(Position::from_xyz(1., 0.0, -3.0), 1.0);

    scene.render_to_file("image.ppm");

    Ok(())
}
