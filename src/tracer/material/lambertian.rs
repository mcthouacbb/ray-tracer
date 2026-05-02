use std::sync::Arc;

use rand::RngExt;

use crate::{
    math::Vec3,
    tracer::{
        material::ScatterResult,
        ray::{Ray, RayHit},
        scene::SceneHit,
        texture::Texture,
    },
};

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
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
        let hit_pt = ray.origin() + ray.dir() * ray_hit.dist();
        let scattered_ray = Ray::new(hit_pt + scene_hit.normal() * 1e-3, scatter_dir);

        Some(ScatterResult::new(
            scattered_ray,
            self.albedo.color(scene_hit.uv(), hit_pt),
        ))
    }
}
