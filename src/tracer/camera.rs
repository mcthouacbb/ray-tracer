use rand::RngExt;

use crate::{math::Vec3, tracer::ray::Ray};

pub struct Camera {
    aspect_ratio: f32,
    vfov: f32,
    focus_dist: f32,
    defocus_angle: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vfov: f32, focus_dist: f32, defocus_angle: f32) -> Self {
        Self {
            aspect_ratio,
            vfov,
            focus_dist,
            defocus_angle,
        }
    }

    pub fn get_ray_dir(&self, u: f32, v: f32, rng: &mut impl RngExt) -> Ray {
        let fov_scale = (self.vfov / 2.0).tan();
        let ray_target =
            self.focus_dist * Vec3::new(u * fov_scale * self.aspect_ratio, v * fov_scale, -1.0);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).tan();
        let ray_origin = defocus_radius * Vec3::random_unit_disk(rng);
        let ray_dir = (ray_target - ray_origin).normalized();
        Ray::new(ray_origin, ray_dir)
    }
}
