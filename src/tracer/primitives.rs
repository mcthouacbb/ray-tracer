use crate::{
    math::Vec3,
    tracer::{
        aabb::AABB,
        ray::{Ray, RayHit},
        scene::InstanceId,
    },
};

pub mod instance;
pub mod sphere;
pub mod triangle;

pub trait Primitive: Sync + Send {
    fn hit(&self, ray: &Ray, instance_id: InstanceId, primitive_id: u32) -> RayHit;
    fn center(&self) -> Vec3;
    fn bounding_box(&self) -> AABB;

    fn get_normal(&self, ray: &Ray, t: f32) -> Vec3;
}
