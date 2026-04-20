use crate::tracer::ray::{Ray, RayHit};

pub trait Hittable {
    fn hit_dist(&self, ray: &Ray) -> f32;
    fn trace(&self, ray: &Ray) -> RayHit;
}
