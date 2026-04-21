use crate::{
    math::Vec3,
    tracer::{
        aabb::AABB,
        ray::{Ray, RayHit},
    },
};

pub trait Hittable: Sync + Send {
    fn trace(&self, ray: &Ray) -> RayHit;
    fn center(&self) -> Vec3;
    fn bounding_box(&self) -> AABB;
}
