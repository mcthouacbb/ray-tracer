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
        let scatter_dir: Vec3 = loop {
            let scatter_dir = ray_hit.normal() + Vec3::random_unit(rng);
            if scatter_dir.sqr_len() > 1e-8 {
                break scatter_dir;
            }
        };
        let scatter_origin = ray.origin() + ray.dir() * ray_hit.dist();
        let scattered_ray = Ray::new(scatter_origin + scatter_dir * 1e-4, scatter_dir);

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
        let scatter_origin = ray.origin() + ray.dir() * ray_hit.dist();
        let scattered_ray = Ray::new(scatter_origin + scatter_dir * 1e-4, scatter_dir);

        Some(ScatterResult::new(scattered_ray, self.albedo))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    fn scatter(&self, ray: &Ray, ray_hit: &RayHit, rng: &mut impl RngExt) -> Option<ScatterResult> {
        let refractive_index = if ray_hit.front_face() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let refracted_dir = ray
            .dir()
            .normalized()
            .refract(&ray_hit.normal(), refractive_index);
        let refracted_origin = ray.origin() + ray.dir() * ray_hit.dist();

        let scattered_ray = Ray::new(refracted_origin + refracted_dir * 1e-4, refracted_dir);

        Some(ScatterResult::new(scattered_ray, Vec3::new(1.0, 1.0, 1.0)))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
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
            Self::Dielectric(dielectric) => dielectric.scatter(ray, ray_hit, rng),
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
}
