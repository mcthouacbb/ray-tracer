use crate::{
    math::Vec3,
    tracer::{
        bvh::{blas::BLAS, blas_instance::BLASInstance, tlas::TLAS},
        hittable::Hittable,
        material::Material,
        primitives::{Primitive, sphere::Sphere, triangle::Triangle},
        ray::{Ray, RayHit},
    },
    transform::Transform,
};

pub struct SubObject<T: Primitive> {
    primitives: Vec<T>,
}

impl<T> SubObject<T>
where
    T: Primitive,
{
    pub fn new(primitives: Vec<T>) -> Self {
        Self { primitives }
    }

    pub fn primitives(&self) -> &[T] {
        &self.primitives
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshId(u32);

#[derive(Debug, Clone, Copy)]
pub enum InstanceId {
    Sphere,
    Mesh(u32),
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
    spheres: (SubObject<Sphere>, Vec<Material>, Option<BLAS>),
    meshes: Vec<(SubObject<Triangle>, BLAS)>,
    blas_instances: Vec<BLASInstance>,
    tlas: Option<TLAS>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            spheres: (SubObject::new(Vec::new()), Vec::new(), None),
            meshes: Vec::new(),
            blas_instances: Vec::new(),
            tlas: None,
        }
    }

    pub fn add_mesh(&mut self, mesh: SubObject<Triangle>) -> MeshId {
        assert!(mesh.primitives().len() > 0);

        let id = self.meshes.len() as u32;
        let blas = BLAS::create(&mesh);
        self.meshes.push((mesh, blas));
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
        InstanceId::Mesh(id)
    }

    pub fn add_sphere(&mut self, sphere: Sphere, material: Material) -> u32 {
        let id = self.spheres.1.len() as u32;
        self.spheres.0.primitives.push(sphere);
        self.spheres.1.push(material);
        id
    }

    pub fn finalize(&mut self) {
        self.spheres.2 = Some(BLAS::create(&self.spheres.0));
        self.tlas = Some(TLAS::create(self));
    }

    pub fn get_mesh(&self, mesh_id: MeshId) -> &SubObject<Triangle> {
        &self.meshes[mesh_id.0 as usize].0
    }

    pub fn get_blas(&self, blas_id: MeshId) -> &BLAS {
        &self.meshes[blas_id.0 as usize].1
    }

    pub fn get_blas_instance(&self, instance_id: InstanceId) -> &BLASInstance {
        if let InstanceId::Mesh(id) = instance_id {
            &self.blas_instances[id as usize]
        } else {
            panic!("Invalid instance_id passed to Scene::get_blas_instance()");
        }
    }

    pub fn get_sphere(&self, id: u32) -> &Sphere {
        &self.spheres.0.primitives[id as usize]
    }

    pub fn get_scene_hit(&self, ray: &Ray, ray_hit: &RayHit) -> SceneHit {
        let instance_id = ray_hit.instance_id();
        match instance_id {
            InstanceId::Sphere => {
                let sphere = self.get_sphere(ray_hit.primitive_id());
                SceneHit::new(
                    ray,
                    sphere.get_normal(ray, ray_hit.dist()),
                    self.spheres.1[ray_hit.primitive_id() as usize],
                )
            }
            InstanceId::Mesh(_) => {
                let instance = self.get_blas_instance(instance_id);
                let triangle =
                    self.get_mesh(instance.mesh_id()).primitives[ray_hit.primitive_id() as usize];
                let raw_normal = triangle.get_normal(ray, ray_hit.dist());
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

        self.spheres.2.as_ref().unwrap().traverse(
            ray,
            &mut ray_hit,
            InstanceId::Sphere,
            &self.spheres.0,
        );

        self.tlas
            .as_ref()
            .unwrap()
            .traverse(ray, &mut ray_hit, self);

        if ray_hit.dist() < f32::INFINITY {
            (ray_hit, Some(self.get_scene_hit(ray, &ray_hit)))
        } else {
            (ray_hit, None)
        }
    }

    pub fn get_instance_ids(&self) -> Vec<InstanceId> {
        let mut result = Vec::with_capacity(self.blas_instances.len());
        for i in 0..self.blas_instances.len() {
            result.push(InstanceId::Mesh(i as u32));
        }
        result
    }
}
