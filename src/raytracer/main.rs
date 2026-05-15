use raytracer::camera::Camera;
use raytracer::errors::RaytracerError;
use raytracer::errors::RaytracerError::IncorrectArguments;
use raytracer::maths::vec3::Position;
use raytracer::plugins::PluginLoader;
use raytracer::rendering::Renderer;
use raytracer::rendering::transform::Transform;
use raytracer::scene::Scene;
use raytracer::scene::preprocessor;
#[cfg(feature = "sfml-preview")]
use raytracer::sfml;
use serde::Deserialize;
use std::env::args;
use std::error::Error;
use std::path::Path;

fn get_config_file() -> Result<(String, bool), RaytracerError> {
    let mut args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        return Err(IncorrectArguments);
    }
    Ok((args.remove(1), args.contains(&"--sfml".to_string())))
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
struct LightDesc {
    kind: String,
    config: Option<ron::Value>,
}

#[derive(Deserialize)]
struct SceneDesc {
    primitives: Vec<PrimitiveDesc>,
    lights: Vec<LightDesc>,
    camera: Camera,
    ambient: Option<f32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let (config_file, is_sfml) = get_config_file()?;

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
    let mut scene = Scene::new(parsed.camera, parsed.ambient.unwrap_or(1.));

    for mut desc in parsed.primitives {
        let primitive =
            loader.load_primitive(&desc.kind, &desc.config.unwrap_or(ron::Value::Unit))?;
        let material = loader.load_material(&desc.material.kind, &desc.material.config)?;
        let position = desc.position.unwrap_or(Position::ZERO);
        desc.transform.translation += position;
        scene.add_object(primitive, material, desc.transform);
    }

    for desc in parsed.lights {
        let light = loader.load_light(&desc.kind, &desc.config.unwrap_or(ron::Value::Unit))?;
        scene.add_light(light);
    }

    let mut renderer = Renderer::from_scene(scene);
    if is_sfml {
        #[cfg(target_os = "macos")]
        {
            eprintln!(
                "SFML preview is currently unstable on macOS. Rendering to image.ppm instead."
            );
            renderer.render_to_file("image.ppm");
            return Ok(());
        }

        #[cfg(all(not(target_os = "macos"), not(feature = "sfml-preview")))]
        return Err(
            "This binary was built without SFML support. Rebuild with --features sfml-preview."
                .into(),
        );

        #[cfg(all(feature = "sfml-preview", not(target_os = "macos")))]
        sfml::SfmlPreview::new(renderer).run();
    } else {
        renderer.render_to_file("image.ppm");
    }

    Ok(())
}
