use ron::Map;

use crate::maths::vec3::Vec3;
use crate::rendering::transform::Transform;

#[inline]
pub fn transform_from_map(m: &Map) -> Transform {
    let xyz = |k| {
        super::get(m, k)
            .and_then(Vec3::from_ron_value)
            .unwrap_or(Vec3::ZERO)
    };
    Transform::new(xyz("translation"), xyz("rotation"))
}

#[inline]
pub fn compose(outer: Transform, inner: Transform) -> Transform {
    Transform::new(
        outer.translation + inner.translation,
        outer.rotation + inner.rotation,
    )
}
