use super::node::BvhNode;
use super::split::*;
use crate::rendering::aabb::Aabb;

pub struct BvhBuilder<'a> {
    primitives_aabbs: &'a [Aabb],
    spatial_indices: Vec<u32>,
    flat_nodes: Vec<BvhNode>,
}

#[derive(Clone, Copy)]
struct IndexRange {
    start_index: usize,
    end_index: usize,
}

impl IndexRange {
    #[inline(always)]
    fn primitive_count(&self) -> u32 {
        (self.end_index - self.start_index) as u32
    }
}

impl<'a> BvhBuilder<'a> {
    pub fn build(aabbs: &'a [Aabb]) -> (Vec<BvhNode>, Vec<u32>) {
        let mut builder = Self {
            primitives_aabbs: aabbs,
            spatial_indices: (0..aabbs.len() as u32).collect(),
            flat_nodes: vec![],
        };
        builder.recursive_build(IndexRange {
            start_index: 0,
            end_index: aabbs.len(),
        });
        (builder.flat_nodes, builder.spatial_indices)
    }

    fn recursive_build(&mut self, range: IndexRange) -> usize {
        let current_node_index = self.flat_nodes.len();
        self.flat_nodes.push(BvhNode::default());

        let node_bounds = self.compute_node_bounds(range);
        if range.primitive_count() <= 4 {
            return self.initialize_leaf_node(current_node_index, node_bounds, range);
        }

        let centroid_bounds = self.compute_centroid_bounds(range);
        let best_split = self.find_optimal_split(range, &node_bounds, &centroid_bounds);

        if best_split.split_cost >= range.primitive_count() as f32 {
            return self.initialize_leaf_node(current_node_index, node_bounds, range);
        }

        let partition_index = self.perform_spatial_partition(range, &best_split, &centroid_bounds);
        self.initialize_internal_node(current_node_index, node_bounds, range, partition_index)
    }

    fn initialize_leaf_node(
        &mut self,
        node_index: usize,
        bounds: Aabb,
        range: IndexRange,
    ) -> usize {
        self.flat_nodes[node_index].set_data(
            bounds,
            range.start_index as u32,
            range.primitive_count(),
        );
        node_index
    }

    fn initialize_internal_node(
        &mut self,
        node_index: usize,
        bounds: Aabb,
        range: IndexRange,
        mid: usize,
    ) -> usize {
        self.recursive_build(IndexRange {
            start_index: range.start_index,
            end_index: mid,
        });
        let right_child_index = self.recursive_build(IndexRange {
            start_index: mid,
            end_index: range.end_index,
        });
        self.flat_nodes[node_index].set_data(bounds, right_child_index as u32, 0);
        node_index
    }

    fn compute_node_bounds(&self, range: IndexRange) -> Aabb {
        let mut merged_bounds = Aabb::EMPTY;
        for i in range.start_index..range.end_index {
            let primitive_index = self.spatial_indices[i] as usize;
            merged_bounds = Aabb::merge(merged_bounds, self.primitives_aabbs[primitive_index]);
        }
        merged_bounds
    }

    fn compute_centroid_bounds(&self, range: IndexRange) -> Aabb {
        let mut centroid_bounds = Aabb::EMPTY;
        for i in range.start_index..range.end_index {
            let primitive_index = self.spatial_indices[i] as usize;
            centroid_bounds =
                centroid_bounds.extend(self.primitives_aabbs[primitive_index].centroid());
        }
        centroid_bounds
    }

    fn find_optimal_split(
        &self,
        range: IndexRange,
        node_bounds: &Aabb,
        centroids: &Aabb,
    ) -> SplitInfo {
        let mut best_split = SplitInfo {
            split_axis: 0,
            split_bin_index: 0,
            split_cost: f32::MAX,
        };
        for axis_index in 0..3 {
            let axis_result = self.evaluate_split_on_axis(range, axis_index, centroids);
            if axis_result.split_cost < best_split.split_cost {
                best_split = axis_result;
            }
        }
        best_split.split_cost =
            1.0 + best_split.split_cost / calculate_aabb_surface_area(node_bounds);
        best_split
    }

    fn evaluate_split_on_axis(
        &self,
        range: IndexRange,
        axis: usize,
        centroids: &Aabb,
    ) -> SplitInfo {
        let min_coord = get_vector_dimension(centroids.min, axis);
        let max_coord = get_vector_dimension(centroids.max, axis);
        if max_coord - min_coord < 1e-6 {
            return SplitInfo {
                split_axis: axis,
                split_bin_index: 0,
                split_cost: f32::MAX,
            };
        }

        let params = AxisPartitionParams {
            partition_axis: axis,
            axis_min_coordinate: min_coord,
            partition_scale: BIN_COUNT as f32 / (max_coord - min_coord),
        };
        let axis_bins = self.distribute_primitives_into_bins(range, &params);
        self.find_best_split_in_bins(axis, &axis_bins)
    }

    fn distribute_primitives_into_bins(
        &self,
        range: IndexRange,
        params: &AxisPartitionParams,
    ) -> [Bin; BIN_COUNT] {
        let mut bins = [Bin::default(); BIN_COUNT];
        for i in range.start_index..range.end_index {
            let primitive_idx = self.spatial_indices[i] as usize;
            let aabb = self.primitives_aabbs[primitive_idx];
            let bin_index = calculate_target_bin_index(aabb.centroid(), params);
            bins[bin_index].bin_primitive_count += 1;
            bins[bin_index].bin_bounds = Aabb::merge(bins[bin_index].bin_bounds, aabb);
        }
        bins
    }

    fn find_best_split_in_bins(&self, axis: usize, bins: &[Bin; BIN_COUNT]) -> SplitInfo {
        let mut best_split = SplitInfo {
            split_axis: axis,
            split_bin_index: 0,
            split_cost: f32::MAX,
        };
        for split_candidate in 1..BIN_COUNT {
            let (left_bounds, left_count) = self.merge_bin_range(bins, 0, split_candidate);
            let (right_bounds, right_count) =
                self.merge_bin_range(bins, split_candidate, BIN_COUNT);

            if left_count == 0 || right_count == 0 {
                continue;
            }
            let total_cost = left_count as f32 * calculate_aabb_surface_area(&left_bounds)
                + right_count as f32 * calculate_aabb_surface_area(&right_bounds);
            if total_cost < best_split.split_cost {
                best_split.split_bin_index = split_candidate;
                best_split.split_cost = total_cost;
            }
        }
        best_split
    }

    #[inline(always)]
    fn merge_bin_range(&self, bins: &[Bin], start: usize, end: usize) -> (Aabb, u32) {
        let (mut merged_aabb, mut merged_count) = (Aabb::EMPTY, 0);
        for i in start..end {
            merged_aabb = Aabb::merge(merged_aabb, bins[i].bin_bounds);
            merged_count += bins[i].bin_primitive_count;
        }
        (merged_aabb, merged_count)
    }

    fn perform_spatial_partition(
        &mut self,
        range: IndexRange,
        split: &SplitInfo,
        centroids: &Aabb,
    ) -> usize {
        let min_coord = get_vector_dimension(centroids.min, split.split_axis);
        let max_coord = get_vector_dimension(centroids.max, split.split_axis);
        let params = AxisPartitionParams {
            partition_axis: split.split_axis,
            axis_min_coordinate: min_coord,
            partition_scale: BIN_COUNT as f32 / (max_coord - min_coord),
        };

        let (mut left_ptr, mut right_ptr) = (range.start_index, range.end_index);
        while left_ptr < right_ptr {
            let primitive_index = self.spatial_indices[left_ptr] as usize;
            let current_bin = calculate_target_bin_index(
                self.primitives_aabbs[primitive_index].centroid(),
                &params,
            );
            if current_bin < split.split_bin_index {
                left_ptr += 1;
            } else {
                right_ptr -= 1;
                self.spatial_indices.swap(left_ptr, right_ptr);
            }
        }
        left_ptr.max(range.start_index + 1).min(range.end_index - 1)
    }
}
