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
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        ray_hit: &RayHit,
        scene_hit: &SceneHit,
        rng: &mut impl RngExt,
    ) -> Option<ScatterResult> {
        let scatter_dir: Vec3 = loop {
            let scatter_dir = scene_hit.normal() + Vec3::random_unit(rng);
            if scatter_dir.sqr_len() > 1e-8 {
                break scatter_dir;
            }
        };
        let scatter_origin = ray.origin() + ray.dir() * ray_hit.dist();
        let scattered_ray = Ray::new(scatter_origin + scene_hit.normal() * 1e-3, scatter_dir);

        Some(ScatterResult::new(scattered_ray, self.albedo))
    }
}
