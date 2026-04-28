use crate::math::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Emissive {
    color: Vec3,
}

impl Emissive {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }

    pub fn emitted(&self) -> Vec3 {
        self.color
    }
}
