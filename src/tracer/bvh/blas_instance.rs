use crate::{
    math::{Mat4, Vec3},
    tracer::{
        aabb::AABB,
        ray::{Ray, RayHit},
        scene::{MeshId, Scene},
    },
    transform::Transform,
};

pub struct BLASInstance {
    blas_id: MeshId,
    transform: Transform,
    aabb: AABB,
    transform_inv: Mat4,
}

impl BLASInstance {
    pub fn new(blas_id: MeshId, scene: &Scene, transform: Transform) -> Self {
        let blas = scene.get_blas(blas_id);

        let mut aabb = AABB::NEG_INF;
        for corner_idx in 0..8 {
            let x = if corner_idx & 1 > 0 {
                blas.bounding_box().min().x()
            } else {
                blas.bounding_box().max().x()
            };
            let y = if corner_idx & 2 > 0 {
                blas.bounding_box().min().y()
            } else {
                blas.bounding_box().max().y()
            };
            let z = if corner_idx & 4 > 0 {
                blas.bounding_box().min().z()
            } else {
                blas.bounding_box().max().z()
            };

            let old_corner = Vec3::new(x, y, z);
            let corner = transform.transform().transform_pos(&old_corner);
            aabb.add_point(corner);
        }

        Self {
            blas_id,
            transform,
            aabb,
            transform_inv: transform.transform_inv(),
        }
    }

    pub fn traverse(&self, ray: &Ray, ray_hit: &mut RayHit, scene: &Scene) {
        let new_ray = Ray::new(
            self.transform_inv.transform_pos(&ray.origin()),
            self.transform_inv.transform_dir(&ray.dir()),
        );
        scene.get_blas(self.blas_id).traverse(
            &new_ray,
            ray_hit,
            scene.get_mesh(self.blas_id).primitives(),
        );
    }

    fn center(&self) -> Vec3 {
        self.aabb.center()
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}
