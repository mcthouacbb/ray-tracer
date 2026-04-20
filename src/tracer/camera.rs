use crate::math::Vec3;

pub struct Camera {
    aspect_ratio: f32,
    vfov: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vfov: f32) -> Self {
        Self { aspect_ratio, vfov }
    }

    pub fn get_ray_dir(&self, u: f32, v: f32) -> Vec3 {
        let fov_scale = (self.vfov / 2.0).tan();
        let ray_dir =
            Vec3::new(u * fov_scale * self.aspect_ratio, v * fov_scale, -1.0).normalized();
        ray_dir.normalized()
    }
}
