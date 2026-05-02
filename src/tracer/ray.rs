use crate::{
    math::{Vec2, Vec3},
    tracer::scene::InstanceId,
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
    instance_id: Option<InstanceId>,
    primitive_id: u32,
    tri_uv: Option<Vec2>,
}

impl RayHit {
    pub const NONE: Self = Self {
        dist: f32::INFINITY,
        instance_id: None,
        primitive_id: 0,
        tri_uv: None,
    };

    pub fn new(
        dist: f32,
        instance_id: InstanceId,
        primitive_id: u32,
        tri_uv: Option<Vec2>,
    ) -> Self {
        assert!(dist < f32::INFINITY);

        Self {
            dist,
            instance_id: Some(instance_id),
            primitive_id,
            tri_uv,
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
        self.instance_id.unwrap()
    }

    pub fn primitive_id(&self) -> u32 {
        assert!(self.dist < f32::INFINITY);
        self.primitive_id
    }

    pub fn tri_uv(&self) -> Option<Vec2> {
        assert!(self.dist < f32::INFINITY);
        self.tri_uv
    }
}
