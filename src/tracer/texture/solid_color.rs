use crate::{
    math::{Vec2, Vec3},
    tracer::texture::Texture,
};

#[derive(Clone)]
pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn color(&self, _uv: Vec2, _p: Vec3) -> Vec3 {
        self.albedo
    }
}
