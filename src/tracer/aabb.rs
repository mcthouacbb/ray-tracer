use core::f32;

use crate::{math::Vec3, tracer::ray::Ray};

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub const NEG_INF: Self = Self::new(
        Vec3::from_value(f32::INFINITY),
        Vec3::from_value(-f32::INFINITY),
    );

    pub const fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.min
    }

    pub fn hit(&self, ray: &Ray) -> f32 {
        let mut tmin = 0.0;
        let mut tmax = f32::INFINITY;

        for axis in 0..3 {
            let t1 = (self.min[axis] - ray.origin()[axis]) / ray.dir()[axis];
            let t2 = (self.max[axis] - ray.origin()[axis]) / ray.dir()[axis];

            tmin = t1.max(tmin).min(t2.max(tmin));
            tmax = t1.min(tmax).max(t2.min(tmax));
        }

        if tmin <= tmax { tmin } else { f32::INFINITY }
    }
}
