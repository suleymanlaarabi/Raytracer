use libloading::{Library, Symbol};
use raytracer::camera::Camera;
use raytracer::primitives::Primitive;
use serde::Deserialize;

use raytracer::errors::RaytracerError;
use raytracer::errors::RaytracerError::IncorrectArguments;
use raytracer::scene::Scene;
use std::collections::HashMap;
use std::env::args;
use std::error::Error;
use std::path::PathBuf;

type CreateFn = fn(&ron::Value) -> Primitive;

fn get_config_file() -> Result<String, RaytracerError> {
    let mut args = args().collect::<Vec<String>>();

    if args.len() != 2 {
        return Err(IncorrectArguments);
    }

    Ok(args.remove(1))
}

#[derive(Deserialize)]
struct PrimitiveDesc {
    kind: String,
    config: ron::Value,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct SceneDesc {
    primitives: Vec<PrimitiveDesc>,
    lights: Vec<ron::Value>,
    camera: Camera,
}

fn plugin_path(kind: &str) -> PathBuf {
    PathBuf::from("plugins").join(format!("libraytracer_{kind}.so"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = get_config_file()?;

    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_file))
        .build()?;

    let parsed = config.try_deserialize::<SceneDesc>()?;

    let mut libraries: HashMap<String, Library> = HashMap::new();
    let mut scene = Scene::new(parsed.camera);

    for desc in &parsed.primitives {
        if !libraries.contains_key(&desc.kind) {
            let lib = unsafe { Library::new(plugin_path(&desc.kind))? };
            libraries.insert(desc.kind.clone(), lib);
        }
        let lib = &libraries[&desc.kind];
        let create: Symbol<CreateFn> = unsafe { lib.get(b"create")? };
        scene.add_primitive(create(&desc.config));
    }

    scene.render_to_file("image.ppm");

    Ok(())
}
