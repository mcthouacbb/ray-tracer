use crate::tracer::ray::{Ray, RayHit};

pub trait Hittable: Sync + Send {
    fn trace(&self, ray: &Ray) -> RayHit;
}
