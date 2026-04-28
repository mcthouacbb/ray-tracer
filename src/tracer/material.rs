mod dielectric;
mod emissive;
mod lambertian;
mod metal;

use rand::RngExt;

use crate::{
    math::Vec3,
    tracer::{
        material::{
            dielectric::Dielectric, emissive::Emissive, lambertian::Lambertian, metal::Metal,
        },
        ray::{Ray, RayHit},
        scene::SceneHit,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct ScatterResult {
    scattered_ray: Ray,
    attenuation: Vec3,
}

impl ScatterResult {
    fn new(scattered_ray: Ray, attenuation: Vec3) -> Self {
        Self {
            scattered_ray,
            attenuation,
        }
    }

    pub fn scattered_ray(&self) -> &Ray {
        &self.scattered_ray
    }

    pub fn attenuation(&self) -> Vec3 {
        self.attenuation
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    Emissive(Emissive),
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        ray_hit: &RayHit,
        scene_hit: &SceneHit,
        rng: &mut impl RngExt,
    ) -> Option<ScatterResult> {
        match self {
            Self::Lambertian(lambert) => lambert.scatter(ray, ray_hit, scene_hit, rng),
            Self::Metal(metal) => metal.scatter(ray, ray_hit, scene_hit, rng),
            Self::Dielectric(dielectric) => dielectric.scatter(ray, ray_hit, scene_hit, rng),
            Self::Emissive(_) => None,
        }
    }

    pub fn emitted(&self) -> Vec3 {
        match self {
            Self::Lambertian(_) => Vec3::ZERO,
            Self::Metal(_) => Vec3::ZERO,
            Self::Dielectric(_) => Vec3::ZERO,
            Self::Emissive(emissive) => emissive.emitted(),
        }
    }

    pub fn new_lambertian(albedo: Vec3) -> Self {
        Self::Lambertian(Lambertian::new(albedo))
    }

    pub fn new_metal(albedo: Vec3, fuzz: f32) -> Self {
        Self::Metal(Metal::new(albedo, fuzz))
    }

    pub fn new_dielectric(refractive_index: f32) -> Self {
        Self::Dielectric(Dielectric::new(refractive_index))
    }

    pub fn new_emissive(color: Vec3) -> Self {
        Self::Emissive(Emissive::new(color))
    }
}
