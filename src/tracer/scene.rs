use std::ops::Deref;

use crate::{
    math::Vec3,
    tracer::{
        bvh::{blas::BLAS, blas_instance::BLASInstance},
        hittable::Hittable,
        material::Material,
        primitives::Primitive,
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

struct GlobalObjects {
    primitives: Vec<Box<dyn Primitive>>,
    materials: Vec<Material>,
}

impl GlobalObjects {
    fn new() -> Self {
        Self {
            primitives: Vec::new(),
            materials: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshId(u32);

#[derive(Debug, Clone, Copy)]
pub struct InstanceId(u32);

impl InstanceId {
    pub const GLOBAL: Self = Self(0);

    fn new(id: u32) -> Self {
        assert!(id != u32::MAX);
        Self(id ^ u32::MAX)
    }

    fn is_global(&self) -> bool {
        self.0 == 0
    }

    fn get_id(&self) -> u32 {
        assert!(!self.is_global());
        self.0 ^ u32::MAX
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
    global: GlobalObjects,
    meshes: Vec<SubObject>,
    blas_list: Vec<BLAS>,
    blas_instances: Vec<BLASInstance>,

    global_blas: Option<BLAS>,
    // TODO: make this a real TLAS
    // tlas: Option<BLAS>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            global: GlobalObjects::new(),
            meshes: Vec::new(),
            blas_list: Vec::new(),
            blas_instances: Vec::new(),
            global_blas: None, // tlas: None,
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
        InstanceId::new(id)
    }

    pub fn add_global_primitive(
        &mut self,
        primitive: Box<dyn Primitive>,
        material: Material,
    ) -> u32 {
        let id = self.global.primitives.len() as u32;
        self.global.primitives.push(primitive);
        self.global.materials.push(material);
        id
    }

    pub fn finalize(&mut self) {
        self.global_blas = Some(BLAS::create(&self.global.primitives));
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
        &self.blas_instances[instance_id.get_id() as usize]
    }

    pub fn get_global_primitive(&self, id: u32) -> &dyn Primitive {
        self.global.primitives[id as usize].deref()
    }

    pub fn get_scene_hit(&self, ray: &Ray, ray_hit: &RayHit) -> SceneHit {
        let instance_id = ray_hit.instance_id();
        if instance_id.is_global() {
            let primitive = self.get_global_primitive(ray_hit.primitive_id());
            SceneHit::new(
                ray,
                primitive.get_normal(ray, ray_hit.dist()),
                self.global.materials[ray_hit.primitive_id() as usize],
            )
        } else {
            let instance = self.get_blas_instance(instance_id);
            let primitive = self.get_mesh(instance.mesh_id()).primitives
                [ray_hit.primitive_id() as usize]
                .deref();
            let raw_normal = primitive.get_normal(ray, ray_hit.dist());
            let normal = instance
                .transform()
                .normal_mat()
                .transform_dir(&raw_normal)
                .normalized();
            SceneHit::new(ray, normal, *instance.material())
        }
    }

    pub fn trace(&self, ray: &Ray) -> (RayHit, Option<SceneHit>) {
        let mut ray_hit = RayHit::NONE;

        self.global_blas.as_ref().unwrap().traverse(
            ray,
            &mut ray_hit,
            InstanceId::GLOBAL,
            &self.global.primitives,
        );

        for (idx, instance) in self.blas_instances.iter().enumerate() {
            instance.trace(ray, &mut ray_hit, InstanceId::new(idx as u32), self);
        }

        if ray_hit.dist() < f32::INFINITY {
            (ray_hit, Some(self.get_scene_hit(ray, &ray_hit)))
        } else {
            (ray_hit, None)
        }
    }
}
