use rand::RngExt;

use crate::{
    math::Vec3,
    tracer::{
        material::ScatterResult,
        ray::{Ray, RayHit},
        scene::SceneHit,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    pub fn reflectance(cos: f32, refractive_index: f32) -> f32 {
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        ray_hit: &RayHit,
        scene_hit: &SceneHit,
        rng: &mut impl RngExt,
    ) -> Option<ScatterResult> {
        let refractive_index = if scene_hit.front_face() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_dir = ray.dir().normalized();

        let cos = -unit_dir.dot(&scene_hit.normal());
        let sin = (1.0 - cos.powi(2)).max(0.0).sqrt();

        let scatter_dir = if refractive_index * sin > 1.0
            || Self::reflectance(cos, refractive_index) > rng.random_range(0.0..=1.0)
        {
            unit_dir.reflect(&scene_hit.normal())
        } else {
            unit_dir.refract(&scene_hit.normal(), refractive_index)
        };
        let scatter_origin = ray.origin() + ray.dir() * ray_hit.dist();

        let scattered_ray = Ray::new(scatter_origin + scene_hit.normal() * 1e-3, scatter_dir);

        Some(ScatterResult::new(scattered_ray, Vec3::new(1.0, 1.0, 1.0)))
    }
}
