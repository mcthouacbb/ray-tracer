use std::sync::Arc;

use crate::{math::Vec3, tracer::texture::Texture};

#[derive(Clone)]
pub struct SpatialChecker {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl SpatialChecker {
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl Texture for SpatialChecker {
    fn color(&self, /*_u: f32, _v: f32, */ p: Vec3) -> Vec3 {
        let x_coord = (p.x() * self.inv_scale).floor() as i32;
        let y_coord = (p.y() * self.inv_scale).floor() as i32;
        let z_coord = (p.z() * self.inv_scale).floor() as i32;
        let even = (x_coord + y_coord + z_coord) % 2 == 0;
        if even {
            self.even.color(p)
        } else {
            self.odd.color(p)
        }
    }
}
