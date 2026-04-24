use ron::{Map, Value};

use crate::rendering::transform::Transform;

use super::transform::{compose, transform_from_map};
use super::{Defs, Error, get, get_str, seq_strings};

pub fn resolve(value: Value, props: &Defs, defs: &Defs) -> Result<Value, Error> {
    match value {
        Value::String(ref s) if s.starts_with('@') => {
            let name = &s[1..];
            props
                .get(name)
                .or_else(|| defs.get(name))
                .cloned()
                .ok_or_else(|| Error::UndefinedRef(name.into()))
        }
        Value::Map(m) => m
            .into_iter()
            .map(|(k, v)| Ok((k, resolve(v, props, defs)?)))
            .collect::<Result<Map, _>>()
            .map(Value::Map),
        Value::Seq(s) => s
            .into_iter()
            .map(|v| resolve(v, props, defs))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Seq),
        other => Ok(other),
    }
}

pub fn expand(
    primitives: &[Value],
    outer_t: Transform,
    props: &Defs,
    defs: &Defs,
    stack: &mut Vec<String>,
) -> Result<Vec<Value>, Error> {
    let mut out = vec![];

    for prim in primitives {
        let Value::Map(map) = prim else {
            return Err(Error::InvalidStructure("primitive must be a map"));
        };
        let kind = get_str(map, "kind")
            .ok_or(Error::InvalidStructure("primitive missing 'kind'"))?
            .to_string();
        let inner_t = get(map, "transform")
            .and_then(|v| {
                if let Value::Map(m) = v {
                    Some(transform_from_map(m))
                } else {
                    None
                }
            })
            .unwrap_or(Transform::ZERO);

        let composed = compose(outer_t, inner_t);

        if let Some(Value::Map(def)) = defs.get(&kind)
            && let Some(Value::Seq(inner)) = get(def, "primitives")
        {
            if stack.contains(&kind) {
                return Err(Error::CyclicDependency({
                    let mut c = stack.clone();
                    c.push(kind);
                    c
                }));
            }
            let decl = get(def, "props").map(seq_strings).unwrap_or_default();
            let callsite =
                get(map, "props").and_then(|v| if let Value::Map(m) = v { Some(m) } else { None });
            let resolved_props: Defs = decl
                .iter()
                .map(|p| -> Result<_, Error> {
                    let raw = callsite
                        .and_then(|m| get(m, p).cloned())
                        .ok_or_else(|| Error::MissingProp(kind.clone(), p.clone()))?;
                    Ok((p.clone(), resolve(raw, props, defs)?))
                })
                .collect::<Result<_, _>>()?;

            stack.push(kind);
            let inner = inner.clone();
            out.extend(expand(&inner, composed, &resolved_props, defs, stack)?);
            stack.pop();
            continue;
        }

        let composed_val = Value::Map(composed.to_map());
        let mut new_map: Map = map
            .iter()
            .map(|(k, v)| {
                let val = if k == &Value::String("transform".into()) {
                    Ok(composed_val.clone())
                } else {
                    resolve(v.clone(), props, defs)
                };
                val.map(|v| (k.clone(), v))
            })
            .collect::<Result<_, _>>()?;

        if get(&new_map, "transform").is_none() {
            new_map.insert(Value::String("transform".into()), composed_val);
        }
        out.push(Value::Map(new_map));
    }

    Ok(out)
}
