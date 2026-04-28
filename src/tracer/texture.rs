pub mod solid_color;
pub use solid_color::*;

use crate::math::Vec3;

pub trait Texture: Sync + Send {
    fn color(&self, /*u: f32, v: f32, */ p: Vec3) -> Vec3;
}
