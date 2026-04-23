use crate::{
    tracer::{
        bvh::{blas::BLAS, blas_instance::BLASInstance},
        hittable::Hittable,
        ray::{Ray, RayHit},
    },
    transform::Transform,
};

pub struct SubObject {
    primitives: Vec<Box<dyn Hittable>>,
}

impl SubObject {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
        }
    }

    pub fn primitives(&self) -> &Vec<Box<dyn Hittable>> {
        &self.primitives
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshId(u32);

#[derive(Debug, Clone, Copy)]
pub struct InstanceId(u32);

pub struct Scene {
    global: SubObject,
    meshes: Vec<SubObject>,
    blas_list: Vec<BLAS>,
    instances: Vec<BLASInstance>,

    // TODO: make this a real TLAS
    tlas: Option<BLAS>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            global: SubObject::new(),
            meshes: Vec::new(),
            blas_list: Vec::new(),
            instances: Vec::new(),
            tlas: None,
        }
    }

    pub fn add_global_object(&mut self, obj: Box<dyn Hittable>) {
        self.global.primitives.push(obj);
    }

    pub fn add_mesh(&mut self, mesh: SubObject) -> MeshId {
        let id = self.meshes.len() as u32;
        self.blas_list.push(BLAS::create(mesh.primitives()));
        self.meshes.push(mesh);
        MeshId(id)
    }

    pub fn add_instance(&mut self, mesh_id: MeshId, transform: Transform) -> InstanceId {
        let id = self.instances.len() as u32;
        self.instances
            .push(BLASInstance::new(mesh_id, self, transform));
        InstanceId(id)
    }

    pub fn finalize(&mut self) {
        self.tlas = Some(BLAS::create(self.global.primitives()));
    }

    pub fn get_mesh(&self, mesh_id: MeshId) -> &SubObject {
        &self.meshes[mesh_id.0 as usize]
    }

    pub fn get_blas(&self, blas_id: MeshId) -> &BLAS {
        &self.blas_list[blas_id.0 as usize]
    }

    pub fn get_instance(&self, instance_id: InstanceId) -> &BLASInstance {
        &self.instances[instance_id.0 as usize]
    }

    pub fn trace(&self, ray: &Ray) -> RayHit {
        let tlas = self.tlas.as_ref().unwrap();
        let mut ray_hit = RayHit::NONE;
        tlas.traverse(ray, &mut ray_hit, self.global.primitives());

        for instance in &self.instances {
            instance.traverse(ray, &mut ray_hit, self);
        }

        ray_hit
    }
}
