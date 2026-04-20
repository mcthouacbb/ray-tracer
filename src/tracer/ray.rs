use crate::math::Vec3;

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
}

impl RayHit {
    pub const NONE: Self = Self {
        dist: f32::INFINITY,
        normal: Vec3::ZERO,
    };

    pub fn new(dist: f32, normal: Vec3) -> Self {
        Self { dist, normal }
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
        self.normal
    }
}
