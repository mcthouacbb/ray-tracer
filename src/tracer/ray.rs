use crate::{
    math::Vec3,
    tracer::{material::Material, scene::InstanceId},
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    dist: f32,
    instance_id: InstanceId,
    primitive_id: u32,
    // uv: (f32, f32),
}

impl RayHit {
    pub const NONE: Self = Self {
        dist: f32::INFINITY,
        instance_id: InstanceId::NONE,
        primitive_id: 0,
    };

    pub fn new(
        dist: f32,
        instance_id: InstanceId,
        primitive_id: u32, /*, uv: (f32, f32)*/
    ) -> Self {
        assert!(dist < f32::INFINITY);

        Self {
            dist,
            instance_id,
            primitive_id,
            // uv,
        }
    }

    pub fn replace_if_closer(&mut self, hit: &Self) {
        if hit.dist < self.dist {
            *self = *hit;
        }
    }

    pub fn dist(&self) -> f32 {
        self.dist
    }

    pub fn instance_id(&self) -> InstanceId {
        assert!(self.dist < f32::INFINITY);
        self.instance_id
    }

    pub fn primitive_id(&self) -> u32 {
        assert!(self.dist < f32::INFINITY);
        self.primitive_id
    }

    /*pub fn uv(&self) -> (f32, f32) {
        self.uv
    }*/
}
