use std::{mem, ops::Range};

use crate::tracer::aabb::AABB;

#[derive(Debug, Clone)]
pub(in crate::tracer::bvh) struct BVHNode {
    pub aabb: AABB,
    pub left: u32,
    pub right: u32,
}

impl BVHNode {
    pub fn is_leaf(&self) -> bool {
        self.right != u32::MAX
    }

    pub fn indices(&self) -> Range<usize> {
        assert!(self.is_leaf());
        self.left as usize..self.right as usize
    }

    pub fn left_right_idx(&self) -> (usize, usize) {
        assert!(!self.is_leaf());
        (self.left as usize, self.left as usize + 1)
    }
}

pub fn binned_sah<const NUM_BINS: usize, T, F: Fn(&T) -> (AABB, usize)>(
    slice: &[T],
    get_data: F,
) -> (f32, usize) {
    let mut bins = [(AABB::NEG_INF, 0u32); NUM_BINS];
    for elem in slice {
        let (bounding_box, pos) = get_data(elem);
        bins[pos].0.expand(&bounding_box);
        bins[pos].1 += 1;
    }

    // this array only needs to be NUM_BINS - 1 in length but
    // rust doesn't support const operations with generics
    let mut right_bins = [(AABB::NEG_INF, 0u32); NUM_BINS];
    for i in (0..NUM_BINS - 1).rev() {
        if i < NUM_BINS - 2 {
            right_bins[i] = right_bins[i + 1];
        }

        right_bins[i].0.expand(&bins[i + 1].0);
        right_bins[i].1 += bins[i + 1].1;
    }

    let mut best_sah = f32::INFINITY;
    let mut best_split_pos = 0;

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
                best_split_pos = i;
            }
        }

        left_aabb.expand(&bins[i].0);
        left_count += bins[i].1;
    }

    (best_sah, best_split_pos)
}

pub fn partition<T, F: Fn(&T) -> bool>(slice: &mut [T], comparator: F) -> usize {
    let mut i = 0;
    let mut j = slice.len() - 1;

    loop {
        while i < j && comparator(&slice[i]) {
            i += 1;
        }

        while i < j && !comparator(&slice[j]) {
            j -= 1;
        }

        if i >= j {
            return i;
        }

        let (left_slice, right_slice) = slice.split_at_mut(j);
        mem::swap(&mut left_slice[i], &mut right_slice[0]);
    }
}
