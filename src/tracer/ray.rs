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

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    t: f32,
    normal: Vec3,
}

impl RayHit {
    pub const NONE: Self = Self {
        t: f32::INFINITY,
        normal: Vec3::ZERO,
    };

    pub fn new(t: f32, normal: Vec3) -> Self {
        Self { t, normal }
    }

    pub fn replace_if_closer(&mut self, hit: &Self) {
        if hit.t < self.t {
            *self = *hit;
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}
