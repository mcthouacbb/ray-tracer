use crate::{
    math::Vec3,
    tracer::{
        aabb::AABB,
        ray::{Ray, RayHit},
        scene::{InstanceId, Scene},
    },
};

pub trait Hittable: Sync + Send {
    fn trace(&self, ray: &Ray, ray_hit: &mut RayHit, instance_id: InstanceId, scene: &Scene);
    fn center(&self) -> Vec3;
    fn bounding_box(&self) -> AABB;
}
