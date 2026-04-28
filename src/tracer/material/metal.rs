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
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        assert!(0.0 <= fuzz && fuzz <= 1.0);
        Self { albedo, fuzz }
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        ray_hit: &RayHit,
        scene_hit: &SceneHit,
        rng: &mut impl RngExt,
    ) -> Option<ScatterResult> {
        let reflected_dir = ray.dir().reflect(&scene_hit.normal()).normalized();
        let scatter_dir = loop {
            let scatter_dir = reflected_dir + self.fuzz * Vec3::random_unit(rng);
            if scatter_dir.sqr_len() > 1e-8 {
                break scatter_dir;
            }
        };
        let scatter_origin = ray.origin() + ray.dir() * ray_hit.dist();
        let scattered_ray = Ray::new(scatter_origin + scene_hit.normal() * 1e-3, scatter_dir);

        Some(ScatterResult::new(scattered_ray, self.albedo))
    }
}
