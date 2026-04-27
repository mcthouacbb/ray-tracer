use core::f32;
use std::{mem, u32};

use crate::tracer::{
    aabb::AABB,
    bvh::bvh_utils::{BVHNode, binned_sah, partition},
    hittable::Hittable,
    ray::{Ray, RayHit},
    scene::{InstanceId, Scene},
};

pub struct TLAS {
    nodes: Vec<BVHNode>,
    instance_ids: Vec<InstanceId>,
}

impl TLAS {
    const NUM_BINS: usize = 16;

    pub fn create(scene: &Scene) -> Self {
        let instance_ids = scene.get_instance_ids();
        let num_instances = instance_ids.len();

        let mut result = Self {
            nodes: Vec::with_capacity(2 * num_instances - 1),
            instance_ids: instance_ids,
        };

        result.nodes.push(BVHNode {
            aabb: AABB::NEG_INF,
            left: 0,
            right: num_instances as u32,
        });
        result.calc_node_bounds(0, scene);

        result.build_bvh(0, scene);

        result
    }

    pub fn bounding_box(&self) -> AABB {
        self.nodes[0].aabb
    }

    pub fn traverse(&self, ray: &Ray, ray_hit: &mut RayHit, scene: &Scene) {
        if self.nodes[0].aabb.hit(ray) < f32::INFINITY {
            self.traverse_impl(0, ray, ray_hit, scene);
        }
    }

    pub fn traverse_impl(&self, node_idx: usize, ray: &Ray, ray_hit: &mut RayHit, scene: &Scene) {
        let node = &self.nodes[node_idx];
        if node.is_leaf() {
            for i in node.indices() {
                let instance_id = self.instance_ids[i];
                scene
                    .get_blas_instance(instance_id)
                    .trace(ray, ray_hit, instance_id, scene);
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
                self.traverse_impl(close_idx, ray, ray_hit, scene);
                if far_dist < f32::INFINITY && far_dist < ray_hit.dist() {
                    self.traverse_impl(far_idx, ray, ray_hit, scene);
                }
            }
        }
    }

    fn calc_node_bounds(&mut self, node_idx: usize, scene: &Scene) {
        let node = &mut self.nodes[node_idx];
        node.aabb = AABB::NEG_INF;
        for i in node.indices() {
            let instance = scene.get_blas_instance(self.instance_ids[i]);
            node.aabb.expand(&instance.bounding_box());
        }
    }

    fn find_split_plane(&self, node_idx: usize, scene: &Scene) -> (usize, usize, f32) {
        let node = &self.nodes[node_idx];
        let mut best_sah = f32::INFINITY;
        let mut best_axis = 0;
        let mut best_split_pos = 0;
        for axis in 0..3 {
            let bin_start = node.aabb.min()[axis];
            let bin_size = node.aabb.extent()[axis] / Self::NUM_BINS as f32;

            let (sah, split_pos) =
                binned_sah::<{ Self::NUM_BINS }, _, _>(&self.instance_ids[node.indices()], |&i| {
                    let instance = scene.get_blas_instance(i);
                    (
                        instance.bounding_box(),
                        (((instance.center()[axis] - bin_start) / bin_size) as usize)
                            .min(Self::NUM_BINS - 1),
                    )
                });

            if sah < best_sah {
                best_sah = sah;
                best_axis = axis;
                best_split_pos = split_pos;
            }
        }

        (best_axis, best_split_pos, best_sah)
    }

    fn partition_primitives(
        &mut self,
        node_idx: usize,
        split_axis: usize,
        split_pos: usize,
        scene: &Scene,
    ) -> u32 {
        let bin_start = self.nodes[node_idx].aabb.min()[split_axis];
        let bin_size = self.nodes[node_idx].aabb.extent()[split_axis] / Self::NUM_BINS as f32;
        partition(
            &mut self.instance_ids[self.nodes[node_idx].indices()],
            |&i| {
                (((scene.get_blas_instance(i).center()[split_axis] - bin_start) / bin_size)
                    as usize)
                    < split_pos
            },
        ) as u32
            + self.nodes[node_idx].left
    }

    fn build_bvh(&mut self, node_idx: usize, scene: &Scene) {
        let (split_axis, split_pos, split_sah) = self.find_split_plane(node_idx, scene);
        let curr_sah =
            self.nodes[node_idx].aabb.surface_area() * self.nodes[node_idx].indices().len() as f32;

        if curr_sah <= split_sah {
            return;
        }

        let right_start = self.partition_primitives(node_idx, split_axis, split_pos, scene);

        let left_child_idx = self.nodes.len();
        self.nodes.push(BVHNode {
            aabb: AABB::NEG_INF,
            left: self.nodes[node_idx].left,
            right: right_start,
        });

        self.nodes.push(BVHNode {
            aabb: AABB::NEG_INF,
            left: right_start,
            right: self.nodes[node_idx].right,
        });

        self.nodes[node_idx].left = left_child_idx as u32;
        self.nodes[node_idx].right = u32::MAX;

        self.calc_node_bounds(left_child_idx, scene);
        self.build_bvh(left_child_idx, scene);

        self.calc_node_bounds(left_child_idx + 1, scene);
        self.build_bvh(left_child_idx + 1, scene);
    }
}
