use std::ops::Deref;

use crate::{
    math::Vec3,
    tracer::{
        bvh::{blas::BLAS, blas_instance::BLASInstance},
        hittable::Hittable,
        material::{self, Material},
        primitives::{Primitive, instance::PrimitiveInstance},
        ray::{Ray, RayHit},
    },
    transform::Transform,
};

pub struct SubObject {
    primitives: Vec<Box<dyn Primitive>>,
}

impl SubObject {
    pub fn new(primitives: Vec<Box<dyn Primitive>>) -> Self {
        Self { primitives }
    }

    pub fn primitives(&self) -> &Vec<Box<dyn Primitive>> {
        &self.primitives
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceKind {
    Blas,
    Primitive,
}

#[derive(Debug, Clone, Copy)]
pub struct InstanceId(InstanceKind, u32);

impl InstanceId {
    pub const NONE: Self = Self(InstanceKind::Blas, u32::MAX);

    fn kind(&self) -> InstanceKind {
        self.0
    }
}

pub struct SceneHit {
    normal: Vec3,
    front_face: bool,
    material: Material,
}

impl SceneHit {
    fn new(ray: &Ray, normal: Vec3, material: Material) -> Self {
        if ray.dir().dot(&normal) > 0.0 {
            Self {
                normal: -normal,
                front_face: false,
                material,
            }
        } else {
            Self {
                normal,
                front_face: true,
                material,
            }
        }
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Scene {
    meshes: Vec<SubObject>,
    blas_list: Vec<BLAS>,
    blas_instances: Vec<BLASInstance>,
    primitive_instances: Vec<PrimitiveInstance>,

    // TODO: make this a real TLAS
    tlas: Option<BLAS>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            blas_list: Vec::new(),
            blas_instances: Vec::new(),
            primitive_instances: Vec::new(),
            tlas: None,
        }
    }

    pub fn add_mesh(&mut self, mesh: SubObject) -> MeshId {
        assert!(mesh.primitives().len() > 0);

        let id = self.meshes.len() as u32;
        self.blas_list.push(BLAS::create(mesh.primitives()));
        self.meshes.push(mesh);
        MeshId(id)
    }

    pub fn add_blas_instance(
        &mut self,
        mesh_id: MeshId,
        transform: Transform,
        material: Material,
    ) -> InstanceId {
        let id = self.blas_instances.len() as u32;
        self.blas_instances
            .push(BLASInstance::new(mesh_id, self, transform, material));
        InstanceId(InstanceKind::Blas, id)
    }

    pub fn add_primitive_instance(
        &mut self,
        primitive: Box<dyn Primitive>,
        transform: Transform,
        material: Material,
    ) -> InstanceId {
        let id = self.primitive_instances.len() as u32;
        self.primitive_instances
            .push(PrimitiveInstance::new(primitive, transform, material));
        InstanceId(InstanceKind::Primitive, id)
    }

    pub fn finalize(&mut self) {
        /*if self.global.primitives().len() > 0 {
            self.tlas = Some(BLAS::create(self.global.primitives()));
        }*/
    }

    pub fn get_mesh(&self, mesh_id: MeshId) -> &SubObject {
        &self.meshes[mesh_id.0 as usize]
    }

    pub fn get_blas(&self, blas_id: MeshId) -> &BLAS {
        &self.blas_list[blas_id.0 as usize]
    }

    pub fn get_blas_instance(&self, instance_id: InstanceId) -> &BLASInstance {
        assert!(instance_id.0 == InstanceKind::Blas);
        &self.blas_instances[instance_id.1 as usize]
    }

    pub fn get_primitive_instance(&self, instance_id: InstanceId) -> &PrimitiveInstance {
        assert!(instance_id.0 == InstanceKind::Primitive);
        &self.primitive_instances[instance_id.1 as usize]
    }

    pub fn get_scene_hit(&self, ray: &Ray, ray_hit: &RayHit) -> SceneHit {
        let instance_id = ray_hit.instance_id();
        match instance_id.kind() {
            InstanceKind::Blas => {
                let instance = self.get_blas_instance(instance_id);
                let inst_ray = instance.transform_ray(ray);

                let primitive = self.get_mesh(instance.mesh_id()).primitives
                    [ray_hit.primitive_id() as usize]
                    .deref();
                let raw_normal = primitive.get_normal(&inst_ray, ray_hit.dist());
                let normal = instance
                    .transform()
                    .normal_mat()
                    .transform_dir(&raw_normal)
                    .normalized();
                SceneHit::new(ray, normal, *instance.material())
            }
            InstanceKind::Primitive => {
                let instance = self.get_primitive_instance(instance_id);
                let inst_ray = instance.transform_ray(ray);
                let raw_normal = instance.primitive().get_normal(&inst_ray, ray_hit.dist());
                let normal = instance
                    .transform()
                    .normal_mat()
                    .transform_dir(&raw_normal)
                    .normalized();

                SceneHit::new(ray, normal, *instance.material())
            }
        }
    }

    pub fn trace(&self, ray: &Ray) -> (RayHit, Option<SceneHit>) {
        let mut ray_hit = RayHit::NONE;

        /*if let Some(tlas) = self.tlas.as_ref() {
            tlas.traverse(ray, &mut ray_hit, self.global.primitives());
        }*/

        for (idx, instance) in self.blas_instances.iter().enumerate() {
            instance.trace(
                ray,
                &mut ray_hit,
                InstanceId(InstanceKind::Blas, idx as u32),
                self,
            );
        }

        for (idx, instance) in self.primitive_instances.iter().enumerate() {
            instance.trace(
                ray,
                &mut ray_hit,
                InstanceId(InstanceKind::Primitive, idx as u32),
                self,
            );
        }

        if ray_hit.dist() < f32::INFINITY {
            (ray_hit, Some(self.get_scene_hit(ray, &ray_hit)))
        } else {
            (ray_hit, None)
        }
    }
}
