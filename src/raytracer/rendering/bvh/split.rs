use crate::maths::vec3::Vec3;
use crate::rendering::aabb::Aabb;

pub const BIN_COUNT: usize = 32;

pub struct SplitInfo {
    pub split_axis: usize,
    pub split_bin_index: usize,
    pub split_cost: f32,
}

#[derive(Clone, Copy, Default)]
pub struct Bin {
    pub bin_bounds: Aabb,
    pub bin_primitive_count: u32,
}

pub struct AxisPartitionParams {
    pub partition_axis: usize,
    pub axis_min_coordinate: f32,
    pub partition_scale: f32,
}

#[inline(always)]
pub fn get_vector_dimension(vector: Vec3, axis: usize) -> f32 {
    match axis {
        0 => vector.x,
        1 => vector.y,
        _ => vector.z,
    }
}

#[inline(always)]
pub fn calculate_aabb_surface_area(bounds: &Aabb) -> f32 {
    let extent = bounds.max - bounds.min;
    2.0 * (extent.x * extent.y + extent.y * extent.z + extent.z * extent.x)
}

#[inline(always)]
pub fn calculate_target_bin_index(centroid: Vec3, params: &AxisPartitionParams) -> usize {
    let coordinate = get_vector_dimension(centroid, params.partition_axis);
    (((coordinate - params.axis_min_coordinate) * params.partition_scale) as usize)
        .min(BIN_COUNT - 1)
}
