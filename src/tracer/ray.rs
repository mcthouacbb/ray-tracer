use crate::{math::Vec3, tracer::material::Material};

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
    normal: Vec3,
    // TODO: could be a MaybeUninit?
    material: Option<Material>,
}

impl RayHit {
    pub const NONE: Self = Self {
        dist: f32::INFINITY,
        normal: Vec3::ZERO,
        material: None,
    };

    pub fn new(dist: f32, normal: Vec3, material: Material) -> Self {
        assert!(dist < f32::INFINITY);

        Self {
            dist,
            normal,
            material: Some(material),
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

    pub fn normal(&self) -> Vec3 {
        assert!(self.dist < f32::INFINITY);
        self.normal
    }

    pub fn material(&self) -> Material {
        assert!(self.dist < f32::INFINITY);
        self.material.unwrap()
    }
}
