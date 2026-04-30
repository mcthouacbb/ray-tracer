use std::f32;

use crate::{
    math::Vec3,
    tracer::{
        aabb::AABB,
        primitives::Primitive,
        ray::{Ray, RayHit},
        scene::InstanceId,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn get_uv(&self, hit_pt: &Vec3) -> (f32, f32) {
        let p = (*hit_pt - self.center) / self.radius;

        let phi = (-p.z()).atan2(p.x()) + f32::consts::PI;
        let theta = (-p.y()).acos();

        (phi / (2.0 * f32::consts::PI), theta / f32::consts::PI)
    }
}

impl Primitive for Sphere {
    fn hit(&self, ray: &Ray, instance_id: InstanceId, primitive_id: u32) -> RayHit {
        let oc = self.center() - ray.origin();
        let a = ray.dir().dot(&ray.dir());
        let b = -2.0 * ray.dir().dot(&oc);
        let c = oc.dot(&oc) - self.radius().powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let dist = if t1 > 0.0 {
                t1
            } else if t2 > 0.0 {
                t2
            } else {
                return RayHit::NONE;
            };

            RayHit::new(dist, instance_id, primitive_id, None)
        } else {
            RayHit::NONE
        }
    }

    fn center(&self) -> Vec3 {
        self.center
    }

    fn bounding_box(&self) -> AABB {
        let min = self.center - Vec3::from_value(self.radius);
        let max = self.center + Vec3::from_value(self.radius);
        AABB::new(min, max)
    }

    fn get_normal(&self, ray: &Ray, t: f32) -> Vec3 {
        let hit_pt = ray.origin() + ray.dir() * t;
        (hit_pt - self.center) / self.radius
    }
}
