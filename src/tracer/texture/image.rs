use image::Rgb32FImage;

use crate::{
    math::{Vec2, Vec3},
    tracer::texture::Texture,
};

pub struct ImageTexture {
    image: Rgb32FImage,
}

impl ImageTexture {
    pub fn new(image: Rgb32FImage) -> Self {
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn color(&self, uv: Vec2, _p: Vec3) -> Vec3 {
        let tx = self.image.width() as f32 * uv.x().clamp(0.0, 1.0);
        let ty = self.image.height() as f32 * (1.0 - uv.y().clamp(0.0, 1.0));
        // TODO: add filtering
        let sample = self.image.get_pixel(
            (tx as u32).clamp(0, self.image.width() - 1),
            (ty as u32).clamp(0, self.image.height() - 1),
        );
        Vec3::new(sample.0[0], sample.0[1], sample.0[2])
    }
}
