use crate::materials::Material;
use crate::primitives::Primitive;
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

type CreatePrimitiveFn = fn(&ron::Value) -> Primitive;
type CreateMaterialFn = fn(&ron::Value) -> Material;

#[derive(Default)]
pub struct PluginLoader {
    primitive_libs: HashMap<String, Library>,
    material_libs: HashMap<String, Library>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            primitive_libs: HashMap::new(),
            material_libs: HashMap::new(),
        }
    }

    fn primitive_path(kind: &str) -> PathBuf {
        PathBuf::from("plugins").join(format!("libraytracer_{kind}.so"))
    }

    fn material_path(kind: &str) -> PathBuf {
        PathBuf::from("plugins").join(format!("libraytracer_material_{kind}.so"))
    }

    pub fn load_primitive(
        &mut self,
        kind: &str,
        config: &ron::Value,
    ) -> Result<Primitive, Box<dyn Error>> {
        if !self.primitive_libs.contains_key(kind) {
            let lib = unsafe { Library::new(Self::primitive_path(kind))? };
            self.primitive_libs.insert(kind.to_string(), lib);
        }
        let lib = &self.primitive_libs[kind];
        let create: Symbol<CreatePrimitiveFn> = unsafe { lib.get(b"create")? };
        Ok(create(config))
    }

    pub fn load_material(
        &mut self,
        kind: &str,
        config: &ron::Value,
    ) -> Result<Material, Box<dyn Error>> {
        if !self.material_libs.contains_key(kind) {
            let lib = unsafe { Library::new(Self::material_path(kind))? };
            self.material_libs.insert(kind.to_string(), lib);
        }
        let lib = &self.material_libs[kind];
        let create: Symbol<CreateMaterialFn> = unsafe { lib.get(b"create")? };
        Ok(create(config))
    }
}
