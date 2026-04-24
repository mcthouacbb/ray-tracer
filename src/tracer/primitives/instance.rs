use std::ops::Deref;

use crate::{
    math::{Mat4, Vec3},
    tracer::{
        aabb::AABB,
        hittable::Hittable,
        material::Material,
        primitives::Primitive,
        ray::{Ray, RayHit},
        scene::{InstanceId, Scene},
    },
    transform::Transform,
};

pub struct PrimitiveInstance {
    primitive: Box<dyn Primitive>,
    transform: Transform,
    aabb: AABB,
    transform_inv: Mat4,
    material: Material,
}

impl PrimitiveInstance {
    pub fn new(primitive: Box<dyn Primitive>, transform: Transform, material: Material) -> Self {
        let mut aabb = AABB::NEG_INF;
        for corner_idx in 0..8 {
            let x = if corner_idx & 1 > 0 {
                primitive.bounding_box().min().x()
            } else {
                primitive.bounding_box().max().x()
            };
            let y = if corner_idx & 2 > 0 {
                primitive.bounding_box().min().y()
            } else {
                primitive.bounding_box().max().y()
            };
            let z = if corner_idx & 4 > 0 {
                primitive.bounding_box().min().z()
            } else {
                primitive.bounding_box().max().z()
            };

            let old_corner = Vec3::new(x, y, z);
            let corner = transform.transform().transform_pos(&old_corner);
            aabb.add_point(corner);
        }

        Self {
            primitive,
            transform,
            aabb,
            transform_inv: transform.transform_inv(),
            material,
        }
    }

    pub fn transform_ray(&self, ray: &Ray) -> Ray {
        Ray::new(
            self.transform_inv.transform_pos(&ray.origin()),
            self.transform_inv.transform_dir(&ray.dir()),
        )
    }

    pub fn primitive(&self) -> &dyn Primitive {
        self.primitive.deref()
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

impl Hittable for PrimitiveInstance {
    fn trace(&self, ray: &Ray, ray_hit: &mut RayHit, instance_id: InstanceId, _scene: &Scene) {
        let hit = self.primitive.hit(&self.transform_ray(ray), instance_id, 0);
        ray_hit.replace_if_closer(&hit);
    }

    fn center(&self) -> Vec3 {
        self.bounding_box().center()
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}
