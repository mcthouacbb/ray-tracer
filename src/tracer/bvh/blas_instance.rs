use crate::{
    math::{Mat4, Vec3},
    tracer::{
        aabb::AABB,
        hittable::Hittable,
        material::Material,
        ray::{Ray, RayHit},
        scene::{InstanceId, MeshId, Scene},
    },
    transform::Transform,
};

pub struct BLASInstance {
    blas_id: MeshId,
    transform: Transform,
    aabb: AABB,
    transform_inv: Mat4,
    material: Material,
}

impl BLASInstance {
    pub fn new(blas_id: MeshId, scene: &Scene, transform: Transform, material: Material) -> Self {
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
            material,
        }
    }

    pub fn transform_ray(&self, ray: &Ray) -> Ray {
        Ray::new(
            self.transform_inv.transform_pos(&ray.origin()),
            self.transform_inv.transform_dir(&ray.dir()),
        )
    }

    pub fn mesh_id(&self) -> MeshId {
        self.blas_id
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

impl Hittable for BLASInstance {
    fn trace(&self, ray: &Ray, ray_hit: &mut RayHit, instance_id: InstanceId, scene: &Scene) {
        scene.get_blas(self.blas_id).traverse(
            &self.transform_ray(ray),
            ray_hit,
            instance_id,
            scene.get_mesh(self.blas_id),
        );
    }

    fn center(&self) -> Vec3 {
        self.bounding_box().center()
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}
