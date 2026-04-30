pub mod solid_color;
pub mod spatial_checker;

pub use solid_color::*;
pub use spatial_checker::*;

use crate::math::Vec3;

pub trait Texture: Sync + Send {
    fn color(&self, /*u: f32, v: f32, */ p: Vec3) -> Vec3;
}
