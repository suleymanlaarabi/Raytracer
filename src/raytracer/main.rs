use raytracer::camera::Camera;
use raytracer::errors::RaytracerError;
use raytracer::errors::RaytracerError::IncorrectArguments;
use raytracer::maths::vec3::Position;
use raytracer::plugins::PluginLoader;
use raytracer::rendering::Renderer;
use raytracer::rendering::transform::Transform;
use raytracer::scene::Scene;
use raytracer::scene::preprocessor;
use serde::Deserialize;
use std::env::args;
use std::error::Error;
use std::path::Path;

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
    config: Option<ron::Value>,
    material: MaterialDesc,
    transform: Transform,
    position: Option<Position>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct SceneDesc {
    props: Option<ron::Value>,
    primitives: Vec<PrimitiveDesc>,
    lights: Vec<ron::Value>,
    camera: Camera,
    imports: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = get_config_file()?;

    let base_dir = Path::new(&config_file).parent().unwrap_or(Path::new("."));
    let raw = std::fs::read_to_string(&config_file)?;

    let preprocessed = preprocessor::preprocess(&raw, base_dir)?;
    let config = config::Config::builder()
        .add_source(config::File::from_str(
            &preprocessed,
            config::FileFormat::Ron,
        ))
        .build()?;

    let parsed = config.try_deserialize::<SceneDesc>()?;

    let mut loader = PluginLoader::new();
    let mut scene = Scene::new(parsed.camera);

    for mut desc in parsed.primitives {
        let primitive =
            loader.load_primitive(&desc.kind, &desc.config.unwrap_or(ron::Value::Unit))?;
        let material = loader.load_material(&desc.material.kind, &desc.material.config)?;
        let position = desc.position.unwrap_or(Position::ZERO);
        desc.transform.translation += position;
        scene.add_object(primitive, material, desc.transform);
    }

    let mut renderer = Renderer::from_scene(scene);

    renderer.render_to_file("image.ppm");

    Ok(())
}
