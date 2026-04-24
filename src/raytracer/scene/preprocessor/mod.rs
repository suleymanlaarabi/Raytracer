mod error;
mod expand;
mod transform;

pub use error::Error;

use std::collections::HashMap;
use std::path::Path;

use ron::{Map, Value};

use crate::rendering::transform::Transform;
use expand::expand;

pub(super) type Defs = HashMap<String, Value>;

pub(super) fn get<'a>(map: &'a Map, key: &str) -> Option<&'a Value> {
    map.get(&Value::String(key.into()))
}

pub(super) fn get_str<'a>(map: &'a Map, key: &str) -> Option<&'a str> {
    get(map, key).and_then(|v| {
        if let Value::String(s) = v {
            Some(s.as_str())
        } else {
            None
        }
    })
}

pub(super) fn seq_strings(v: &Value) -> Vec<String> {
    if let Value::Seq(s) = v {
        s.iter()
            .filter_map(|v| {
                if let Value::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![]
    }
}

fn collect_defs(map: &Map) -> Defs {
    map.iter()
        .filter_map(|(k, v)| {
            if let Value::String(s) = k {
                Some((s.clone(), v.clone()))
            } else {
                None
            }
        })
        .collect()
}

fn load_imports(root: &Map, base_dir: &Path) -> Result<Defs, Error> {
    let Some(Value::Seq(imports)) = get(root, "imports") else {
        return Ok(HashMap::new());
    };
    let mut defs = HashMap::new();
    for path in seq_strings(&Value::Seq(imports.clone())) {
        let content = std::fs::read_to_string(base_dir.join(&path))?;
        if let Value::Map(m) = ron::from_str::<Value>(&content)? {
            if let Some(Value::Map(d)) = get(&m, "components") {
                defs.extend(collect_defs(d));
            }
        }
    }
    Ok(defs)
}

pub fn preprocess(source: &str, base_dir: &Path) -> Result<String, Error> {
    let Value::Map(mut root) = ron::from_str::<Value>(source)? else {
        return Ok(source.to_string());
    };

    if get(&root, "components").is_none() {
        return Ok(source.to_string());
    }

    let mut defs = load_imports(&root, base_dir)?;

    if let Some(Value::Map(map)) = root.remove(&Value::String("components".into())) {
        defs.extend(collect_defs(&map));
    }

    root.remove(&Value::String("imports".into()));

    let primitives = get(&root, "primitives")
        .and_then(|v| {
            if let Value::Seq(s) = v {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_default();

    let expanded = expand(
        &primitives,
        Transform::ZERO,
        &HashMap::new(),
        &defs,
        &mut vec![],
    )?;
    root.insert(Value::String("primitives".into()), Value::Seq(expanded));

    if get(&root, "lights").is_none() {
        root.insert(Value::String("lights".into()), Value::Seq(vec![]));
    }

    Ok(ron::to_string(&Value::Map(root))?)
}
