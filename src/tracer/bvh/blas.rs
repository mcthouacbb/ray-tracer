use core::f32;
use std::{
    mem,
    ops::{Deref, Range},
    u32,
};

use crate::tracer::{
    aabb::AABB,
    hittable::Hittable,
    ray::{Ray, RayHit},
};

#[derive(Debug, Clone)]
struct BLASNode {
    aabb: AABB,
    left: u32,
    right: u32,
}

impl BLASNode {
    fn is_leaf(&self) -> bool {
        self.right != u32::MAX
    }

    fn primitives(&self) -> Range<usize> {
        assert!(self.is_leaf());
        self.left as usize..self.right as usize
    }

    fn left_right_idx(&self) -> (usize, usize) {
        assert!(!self.is_leaf());
        (self.left as usize, self.left as usize + 1)
    }
}

pub struct BLAS {
    nodes: Vec<BLASNode>,
    primitive_indices: Vec<u32>,
}

impl BLAS {
    pub fn create(primitives: &[Box<dyn Hittable>]) -> Self {
        let mut result = Self {
            nodes: Vec::with_capacity(2 * primitives.len() - 1),
            primitive_indices: Vec::with_capacity(primitives.len()),
        };

        for i in 0..primitives.len() {
            result.primitive_indices.push(i as u32);
        }

        result.nodes.push(BLASNode {
            aabb: AABB::NEG_INF,
            left: 0,
            right: primitives.len() as u32,
        });
        result.calc_node_bounds(0, primitives);

        result.build_bvh(0, primitives);

        result
    }

    pub fn traverse(&self, ray: &Ray, ray_hit: &mut RayHit, primitives: &[Box<dyn Hittable>]) {
        if self.nodes[0].aabb.hit(ray) < f32::INFINITY {
            self.traverse_impl(0, ray, ray_hit, primitives);
        }
    }

    pub fn traverse_impl(
        &self,
        node_idx: usize,
        ray: &Ray,
        ray_hit: &mut RayHit,
        primitives: &[Box<dyn Hittable>],
    ) {
        let node = &self.nodes[node_idx];
        if node.is_leaf() {
            for i in node.primitives() {
                let hit = primitives[self.primitive_indices[i] as usize].trace(ray);
                ray_hit.replace_if_closer(&hit);
            }
        } else {
            let (mut close_idx, mut far_idx) = node.left_right_idx();
            let mut close_dist = self.nodes[close_idx].aabb.hit(ray);
            let mut far_dist = self.nodes[far_idx].aabb.hit(ray);
            if close_dist > far_dist {
                mem::swap(&mut close_idx, &mut far_idx);
                mem::swap(&mut close_dist, &mut far_dist);
            }

            if close_dist < f32::INFINITY && close_dist < ray_hit.dist() {
                self.traverse_impl(close_idx, ray, ray_hit, primitives);
                if far_dist < f32::INFINITY && far_dist < ray_hit.dist() {
                    self.traverse_impl(far_idx, ray, ray_hit, primitives);
                }
            }
        }
    }

    fn calc_node_bounds(&mut self, node_idx: usize, primitives: &[Box<dyn Hittable>]) {
        let node = &mut self.nodes[node_idx];
        node.aabb = AABB::NEG_INF;
        for i in node.primitives() {
            let primitive = primitives[self.primitive_indices[i] as usize].deref();
            node.aabb.expand(&primitive.bounding_box());
        }
    }

    fn find_split_plane(
        &self,
        node_idx: usize,
        primitives: &[Box<dyn Hittable>],
    ) -> (usize, f32, f32) {
        let node = &self.nodes[node_idx];
        let mut best_sah = f32::INFINITY;
        let mut best_axis = 0;
        let mut best_split_pos = 0.0;
        for axis in 0..3 {
            const NUM_BINS: usize = 16;

            let mut bins = [(AABB::NEG_INF, 0u32); NUM_BINS];
            let bin_start = node.aabb.min()[axis];
            let bin_size = node.aabb.extent()[axis] / NUM_BINS as f32;

            for i in node.primitives() {
                let primitive = primitives[self.primitive_indices[i] as usize].deref();
                let pos = primitive.center()[axis];
                let bin = ((pos - bin_start) / bin_size) as usize;
                bins[bin.min(NUM_BINS - 1)]
                    .0
                    .expand(&primitive.bounding_box());

                bins[bin.min(NUM_BINS - 1)].1 += 1;
            }

            let mut right_bins = [(AABB::NEG_INF, 0u32); NUM_BINS - 1];
            for i in (0..NUM_BINS - 1).rev() {
                if i < NUM_BINS - 2 {
                    right_bins[i] = right_bins[i + 1];
                }

                right_bins[i].0.expand(&bins[i + 1].0);
                right_bins[i].1 += bins[i + 1].1;
            }

            let mut left_aabb = bins[0].0;
            let mut left_count = bins[0].1;
            for i in 1..NUM_BINS {
                let right_aabb = right_bins[i - 1].0;
                let right_count = right_bins[i - 1].1;

                if right_count > 0 && left_count > 0 {
                    let sah = left_aabb.surface_area() * left_count as f32
                        + right_aabb.surface_area() * right_count as f32;

                    if sah < best_sah {
                        best_sah = sah;
                        best_axis = axis;
                        best_split_pos = bin_start + bin_size * i as f32;
                    }
                }

                left_aabb.expand(&bins[i].0);
                left_count += bins[i].1;
            }
        }

        (best_axis, best_split_pos, best_sah)
    }

    pub fn partition_primitives(
        &mut self,
        node_idx: usize,
        split_axis: usize,
        split_pos: f32,
        primitives: &[Box<dyn Hittable>],
    ) -> u32 {
        let mut i = self.nodes[node_idx].left as usize;
        let mut j = self.nodes[node_idx].right as usize - 1;
        loop {
            while i < j
                && primitives[self.primitive_indices[i] as usize].center()[split_axis] < split_pos
            {
                i += 1;
            }

            while i < j
                && primitives[self.primitive_indices[j] as usize].center()[split_axis] >= split_pos
            {
                j -= 1;
            }

            if i >= j {
                return i as u32;
            }

            let (left_slice, right_slice) = self.primitive_indices.split_at_mut(j);
            mem::swap(&mut left_slice[i], &mut right_slice[0]);
        }
    }

    fn build_bvh(&mut self, node_idx: usize, primitives: &[Box<dyn Hittable>]) {
        let (split_axis, split_pos, split_sah) = self.find_split_plane(node_idx, primitives);
        let curr_sah = self.nodes[node_idx].aabb.surface_area()
            * self.nodes[node_idx].primitives().len() as f32;

        if curr_sah <= split_sah {
            return;
        }

        let right_start = self.partition_primitives(node_idx, split_axis, split_pos, primitives);

        let left_child_idx = self.nodes.len();
        self.nodes.push(BLASNode {
            aabb: AABB::NEG_INF,
            left: self.nodes[node_idx].left,
            right: right_start,
        });

        self.nodes.push(BLASNode {
            aabb: AABB::NEG_INF,
            left: right_start,
            right: self.nodes[node_idx].right,
        });

        self.nodes[node_idx].left = left_child_idx as u32;
        self.nodes[node_idx].right = u32::MAX;

        self.calc_node_bounds(left_child_idx, primitives);
        self.build_bvh(left_child_idx, primitives);

        self.calc_node_bounds(left_child_idx + 1, primitives);
        self.build_bvh(left_child_idx + 1, primitives);
    }
}
