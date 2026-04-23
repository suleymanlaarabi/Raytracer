use raytracer::camera::Camera;
use raytracer::errors::RaytracerError;
use raytracer::errors::RaytracerError::IncorrectArguments;
use raytracer::maths::vec3::Position;
use raytracer::plugins::PluginLoader;
use raytracer::rendering::transform::Transform;
use raytracer::scene::Scene;
use serde::Deserialize;
use std::env::args;
use std::error::Error;

fn get_config_file() -> Result<String, RaytracerError> {
    let mut args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(IncorrectArguments);
    }
    Ok(args.remove(1))
}

#[derive(Deserialize)]
struct MaterialDesc {
    kind: String,
    config: ron::Value,
}

#[derive(Deserialize)]
struct PrimitiveDesc {
    kind: String,
    config: ron::Value,
    material: MaterialDesc,
    transform: Transform,
    position: Option<Position>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct SceneDesc {
    primitives: Vec<PrimitiveDesc>,
    lights: Vec<ron::Value>,
    camera: Camera,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = get_config_file()?;

    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_file))
        .build()?;

    let parsed = config.try_deserialize::<SceneDesc>()?;

    let mut loader = PluginLoader::new();
    let mut scene = Scene::new(parsed.camera);

    for mut desc in parsed.primitives {
        let primitive = loader.load_primitive(&desc.kind, &desc.config)?;
        let material = loader.load_material(&desc.material.kind, &desc.material.config)?;
        let position = desc.position.unwrap_or(Position::ZERO);
        desc.transform.translation += position;
        scene.add_object(primitive, material, desc.transform);
    }

    scene.render_to_file("image.ppm");

    Ok(())
}
