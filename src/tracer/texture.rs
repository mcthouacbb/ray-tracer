pub mod image;
pub mod solid_color;
pub mod spatial_checker;

pub use image::*;
pub use solid_color::*;
pub use spatial_checker::*;

use crate::math::{Vec2, Vec3};

pub trait Texture: Sync + Send {
    fn color(&self, uv: Vec2, p: Vec3) -> Vec3;
}
