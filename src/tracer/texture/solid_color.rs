use crate::{math::Vec3, tracer::texture::Texture};

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
    fn color(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.albedo
    }
}
