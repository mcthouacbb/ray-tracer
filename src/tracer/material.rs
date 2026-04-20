use rand::RngExt;

use crate::{
    math::Vec3,
    tracer::ray::{Ray, RayHit},
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
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    fn scatter(&self, ray: &Ray, ray_hit: &RayHit, rng: &mut impl RngExt) -> Option<ScatterResult> {
        let scatter_direction: Vec3 = loop {
            let scatter_direction = ray_hit.normal() + Vec3::random_unit(rng);
            if scatter_direction.sqr_len() > 1e-8 {
                break scatter_direction;
            }
        };
        let scatter_origin = ray.origin() + ray.dir() * ray_hit.dist() + ray_hit.normal() * 1e-4;
        let scattered_ray = Ray::new(scatter_origin, scatter_direction);

        Some(ScatterResult::new(scattered_ray, self.albedo))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    fn new(albedo: Vec3, fuzz: f32) -> Self {
        assert!(0.0 <= fuzz && fuzz <= 1.0);
        Self { albedo, fuzz }
    }

    fn scatter(&self, ray: &Ray, ray_hit: &RayHit, rng: &mut impl RngExt) -> Option<ScatterResult> {
        let reflected_dir = ray.dir().reflect(&ray_hit.normal()).normalized();
        let scatter_dir = loop {
            let scatter_dir = reflected_dir + self.fuzz * Vec3::random_unit(rng);
            if scatter_dir.sqr_len() > 1e-8 {
                break scatter_dir;
            }
        };
        let scatter_origin = ray.origin() + ray.dir() * ray_hit.dist() + ray_hit.normal() * 1e-4;
        let scattered_ray = Ray::new(scatter_origin, scatter_dir);

        Some(ScatterResult::new(scattered_ray, self.albedo))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        ray_hit: &RayHit,
        rng: &mut impl RngExt,
    ) -> Option<ScatterResult> {
        match self {
            Self::Lambertian(lambert) => lambert.scatter(ray, ray_hit, rng),
            Self::Metal(metal) => metal.scatter(ray, ray_hit, rng),
        }
    }

    pub fn new_lambertian(albedo: Vec3) -> Self {
        Self::Lambertian(Lambertian::new(albedo))
    }

    pub fn new_metal(albedo: Vec3, fuzz: f32) -> Self {
        Self::Metal(Metal::new(albedo, fuzz))
    }
}
