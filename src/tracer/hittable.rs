use crate::tracer::ray::{Ray, RayHit};

pub trait Hittable {
    fn trace(&self, ray: &Ray) -> RayHit;
}
