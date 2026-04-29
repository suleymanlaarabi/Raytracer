pub mod builder;
pub mod node;
pub mod split;

use self::builder::BvhBuilder;
pub use self::node::BvhNode;
use crate::materials::CanShade;
use crate::maths::vec3::Vec3;
use crate::rendering::ray::{HitRecord, Ray};
use crate::scene::Object;

pub struct Bvh {
    flat_nodes: Vec<BvhNode>,
    spatial_indices: Vec<u32>,
}

pub struct RayContext<'a> {
    pub ray: &'a Ray,
    pub inverse_direction: Vec3,
}

impl Bvh {
    pub fn build(objects: &[Object]) -> Self {
        if objects.is_empty() {
            return Self {
                flat_nodes: vec![],
                spatial_indices: vec![],
            };
        }
        let primitives_aabbs: Vec<_> = objects.iter().map(|(p, _, t)| p.aabb(t)).collect();
        let (nodes, indices) = BvhBuilder::build(&primitives_aabbs);
        Self {
            flat_nodes: nodes,
            spatial_indices: indices,
        }
    }

    #[inline]
    pub fn traverse<'a>(
        &self,
        ray: &'a Ray,
        inv: Vec3,
        objects: &'a [Object],
    ) -> Option<(HitRecord, &'a dyn CanShade)> {
        if self.flat_nodes.is_empty() {
            return None;
        }
        let context = RayContext {
            ray,
            inverse_direction: inv,
        };
        let mut state = TraversalState::new();

        while let Some(node_index) = state.pop_node_index() {
            let node = &self.flat_nodes[node_index];
            if node.is_leaf() {
                self.intersect_leaf_primitives(node, &context, objects, &mut state);
            } else {
                self.intersect_internal_node(node, node_index, &context, &mut state);
            }
        }
        state.closest_hit
    }

    #[inline]
    pub fn hit_any(&self, ray: &Ray, inv: Vec3, max_t: f32, objects: &[Object]) -> bool {
        if self.flat_nodes.is_empty() {
            return false;
        }
        let context = RayContext {
            ray,
            inverse_direction: inv,
        };
        let (mut stack, mut stack_top) = ([0u32; 64], 1usize);
        stack[0] = 0;

        while stack_top > 0 {
            stack_top -= 1;
            let node_index = stack[stack_top] as usize;
            let node = &self.flat_nodes[node_index];
            if node.is_leaf() {
                if self.check_leaf_occlusion(node, &context, objects, max_t) {
                    return true;
                }
            } else {
                self.push_internal_children(
                    node,
                    node_index,
                    &context,
                    &mut stack,
                    &mut stack_top,
                    max_t,
                );
            }
        }
        false
    }

    #[inline(always)]
    fn intersect_leaf_primitives<'a>(
        &self,
        node: &BvhNode,
        context: &RayContext,
        objects: &'a [Object],
        state: &mut TraversalState<'a>,
    ) {
        for i in node.data0..node.data0 + node.count {
            let primitive_index = self.spatial_indices[i as usize] as usize;
            let (primitive, material, transform) = &objects[primitive_index];
            if let Some(hit) = context.ray.hit(primitive.as_ref(), transform)
                && hit.t < state.max_hit_distance
            {
                state.max_hit_distance = hit.t;
                state.closest_hit = Some((hit, material.as_ref()));
            }
        }
    }

    #[inline(always)]
    fn intersect_internal_node(
        &self,
        node: &BvhNode,
        node_index: usize,
        context: &RayContext,
        state: &mut TraversalState,
    ) {
        let left_child_index = node_index + 1;
        let right_child_index = node.data0 as usize;

        let left_distance = self.flat_nodes[left_child_index]
            .aabb()
            .intersect(context.ray.position, context.inverse_direction);
        let right_distance = self.flat_nodes[right_child_index]
            .aabb()
            .intersect(context.ray.position, context.inverse_direction);

        match (left_distance, right_distance) {
            (Some(d_left), Some(d_right)) => self.push_ordered_children(
                left_child_index,
                d_left,
                right_child_index,
                d_right,
                state,
            ),
            (Some(d_left), _) if d_left < state.max_hit_distance => {
                state.push_node_index(left_child_index as u32)
            }
            (_, Some(d_right)) if d_right < state.max_hit_distance => {
                state.push_node_index(right_child_index as u32)
            }
            _ => {}
        }
    }

    #[inline(always)]
    fn push_ordered_children(
        &self,
        left_idx: usize,
        left_dist: f32,
        right_idx: usize,
        right_dist: f32,
        state: &mut TraversalState,
    ) {
        let (near_idx, near_dist, far_idx, far_dist) = if left_dist <= right_dist {
            (left_idx, left_dist, right_idx, right_dist)
        } else {
            (right_idx, right_dist, left_idx, left_dist)
        };
        if far_dist < state.max_hit_distance {
            state.push_node_index(far_idx as u32);
        }
        if near_dist < state.max_hit_distance {
            state.push_node_index(near_idx as u32);
        }
    }

    #[inline(always)]
    fn check_leaf_occlusion(
        &self,
        node: &BvhNode,
        context: &RayContext,
        objects: &[Object],
        max_distance: f32,
    ) -> bool {
        for i in node.data0..node.data0 + node.count {
            let primitive_index = self.spatial_indices[i as usize] as usize;
            let (primitive, _, transform) = &objects[primitive_index];
            if let Some(hit) = context.ray.hit(primitive.as_ref(), transform)
                && hit.t < max_distance
            {
                return true;
            }
        }
        false
    }

    #[inline(always)]
    fn push_internal_children(
        &self,
        node: &BvhNode,
        idx: usize,
        ctx: &RayContext,
        stack: &mut [u32; 64],
        top: &mut usize,
        max_t: f32,
    ) {
        let left_idx = idx + 1;
        let right_idx = node.data0 as usize;
        let d_left = self.flat_nodes[left_idx]
            .aabb()
            .intersect(ctx.ray.position, ctx.inverse_direction);
        let d_right = self.flat_nodes[right_idx]
            .aabb()
            .intersect(ctx.ray.position, ctx.inverse_direction);
        if let Some(t) = d_left
            && t < max_t
        {
            stack[*top] = left_idx as u32;
            *top += 1;
        }
        if let Some(t) = d_right
            && t < max_t
        {
            stack[*top] = right_idx as u32;
            *top += 1;
        }
    }
}

struct TraversalState<'a> {
    node_index_stack: [u32; 64],
    stack_top: usize,
    max_hit_distance: f32,
    closest_hit: Option<(HitRecord, &'a dyn CanShade)>,
}

impl<'a> TraversalState<'a> {
    #[inline(always)]
    fn new() -> Self {
        let mut initial_stack = [0; 64];
        initial_stack[0] = 0;
        Self {
            node_index_stack: initial_stack,
            stack_top: 1,
            max_hit_distance: f32::MAX,
            closest_hit: None,
        }
    }
    #[inline(always)]
    fn pop_node_index(&mut self) -> Option<usize> {
        if self.stack_top == 0 {
            None
        } else {
            self.stack_top -= 1;
            Some(self.node_index_stack[self.stack_top] as usize)
        }
    }
    #[inline(always)]
    fn push_node_index(&mut self, index: u32) {
        self.node_index_stack[self.stack_top] = index;
        self.stack_top += 1;
    }
}
