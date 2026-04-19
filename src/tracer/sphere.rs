use crate::math::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}
