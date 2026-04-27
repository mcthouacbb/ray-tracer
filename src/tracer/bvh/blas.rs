use core::f32;
use std::{mem, ops::Range, u32};

use crate::tracer::{
    aabb::AABB,
    bvh::bvh_utils::{binned_sah, partition},
    primitives::Primitive,
    ray::{Ray, RayHit},
    scene::{InstanceId, SubObject},
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
    const NUM_BINS: usize = 16;

    pub fn create<T: Primitive>(object: &SubObject<T>) -> Self {
        assert!(object.primitives().len() > 0);

        let mut result = Self {
            nodes: Vec::with_capacity(2 * object.primitives().len() - 1),
            primitive_indices: Vec::with_capacity(object.primitives().len()),
        };

        for i in 0..object.primitives().len() {
            result.primitive_indices.push(i as u32);
        }

        result.nodes.push(BLASNode {
            aabb: AABB::NEG_INF,
            left: 0,
            right: object.primitives().len() as u32,
        });
        result.calc_node_bounds(0, object);

        result.build_bvh(0, object);

        result
    }

    pub fn bounding_box(&self) -> AABB {
        self.nodes[0].aabb
    }

    pub fn traverse<T: Primitive>(
        &self,
        ray: &Ray,
        ray_hit: &mut RayHit,
        instance_id: InstanceId,
        object: &SubObject<T>,
    ) {
        if self.nodes[0].aabb.hit(ray) < f32::INFINITY {
            self.traverse_impl(0, ray, ray_hit, instance_id, object);
        }
    }

    pub fn traverse_impl<T: Primitive>(
        &self,
        node_idx: usize,
        ray: &Ray,
        ray_hit: &mut RayHit,
        instance_id: InstanceId,
        object: &SubObject<T>,
    ) {
        let node = &self.nodes[node_idx];
        if node.is_leaf() {
            for i in node.primitives() {
                let prim_id = self.primitive_indices[i];
                let hit = object.primitives()[prim_id as usize].hit(ray, instance_id, prim_id);
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
                self.traverse_impl(close_idx, ray, ray_hit, instance_id, object);
                if far_dist < f32::INFINITY && far_dist < ray_hit.dist() {
                    self.traverse_impl(far_idx, ray, ray_hit, instance_id, object);
                }
            }
        }
    }

    fn calc_node_bounds<T: Primitive>(&mut self, node_idx: usize, object: &SubObject<T>) {
        let node = &mut self.nodes[node_idx];
        node.aabb = AABB::NEG_INF;
        for i in node.primitives() {
            let primitive = &object.primitives()[self.primitive_indices[i] as usize];
            node.aabb.expand(&primitive.bounding_box());
        }
    }

    fn find_split_plane<T: Primitive>(
        &self,
        node_idx: usize,
        object: &SubObject<T>,
    ) -> (usize, usize, f32) {
        let node = &self.nodes[node_idx];
        let mut best_sah = f32::INFINITY;
        let mut best_axis = 0;
        let mut best_split_pos = 0;
        for axis in 0..3 {
            let bin_start = node.aabb.min()[axis];
            let bin_size = node.aabb.extent()[axis] / Self::NUM_BINS as f32;

            let (sah, split_pos) = binned_sah::<{ Self::NUM_BINS }, _, _>(
                &self.primitive_indices[node.primitives()],
                |&i| {
                    let primitive = &object.primitives()[i as usize];
                    (
                        primitive.bounding_box(),
                        (((primitive.center()[axis] - bin_start) / bin_size) as usize)
                            .min(Self::NUM_BINS - 1),
                    )
                },
            );

            if sah < best_sah {
                best_sah = sah;
                best_axis = axis;
                best_split_pos = split_pos;
            }
        }

        (best_axis, best_split_pos, best_sah)
    }

    fn partition_primitives<T: Primitive>(
        &mut self,
        node_idx: usize,
        split_axis: usize,
        split_pos: usize,
        object: &SubObject<T>,
    ) -> u32 {
        let bin_start = self.nodes[node_idx].aabb.min()[split_axis];
        let bin_size = self.nodes[node_idx].aabb.extent()[split_axis] / Self::NUM_BINS as f32;
        partition(
            &mut self.primitive_indices[self.nodes[node_idx].primitives()],
            |&i| {
                (((object.primitives()[i as usize].center()[split_axis] - bin_start) / bin_size)
                    as usize)
                    < split_pos
            },
        ) as u32
            + self.nodes[node_idx].left
    }

    fn build_bvh<T: Primitive>(&mut self, node_idx: usize, object: &SubObject<T>) {
        let (split_axis, split_pos, split_sah) = self.find_split_plane(node_idx, object);
        let curr_sah = self.nodes[node_idx].aabb.surface_area()
            * self.nodes[node_idx].primitives().len() as f32;

        if curr_sah <= split_sah {
            return;
        }

        let right_start = self.partition_primitives(node_idx, split_axis, split_pos, object);

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

        self.calc_node_bounds(left_child_idx, object);
        self.build_bvh(left_child_idx, object);

        self.calc_node_bounds(left_child_idx + 1, object);
        self.build_bvh(left_child_idx + 1, object);
    }
}
