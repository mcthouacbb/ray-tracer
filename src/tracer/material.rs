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
struct Lambertian {
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
        let scattered_ray = Ray::new(scatter_origin + ray_hit.normal() * 1e-3, scatter_dir);

        Some(ScatterResult::new(scattered_ray, self.albedo))
    }
}

#[derive(Debug, Clone, Copy)]
struct Metal {
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
        let scattered_ray = Ray::new(scatter_origin + ray_hit.normal() * 1e-3, scatter_dir);

        Some(ScatterResult::new(scattered_ray, self.albedo))
    }
}

#[derive(Debug, Clone, Copy)]
struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cos: f32, refractive_index: f32) -> f32 {
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }

    fn scatter(&self, ray: &Ray, ray_hit: &RayHit, rng: &mut impl RngExt) -> Option<ScatterResult> {
        let refractive_index = if ray_hit.front_face() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_dir = ray.dir().normalized();

        let cos = -unit_dir.dot(&ray_hit.normal());
        let sin = (1.0 - cos.powi(2)).max(0.0).sqrt();

        let scatter_dir = if refractive_index * sin > 1.0
            || Self::reflectance(cos, refractive_index) > rng.random_range(0.0..=1.0)
        {
            unit_dir.reflect(&ray_hit.normal())
        } else {
            unit_dir.refract(&ray_hit.normal(), refractive_index)
        };
        let scatter_origin = ray.origin() + ray.dir() * ray_hit.dist();

        let scattered_ray = Ray::new(scatter_origin + ray_hit.normal() * 1e-3, scatter_dir);

        Some(ScatterResult::new(scattered_ray, Vec3::new(1.0, 1.0, 1.0)))
    }
}

#[derive(Debug, Clone, Copy)]
struct Emissive {
    color: Vec3,
}

impl Emissive {
    fn new(color: Vec3) -> Self {
        Self { color }
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
        rng: &mut impl RngExt,
    ) -> Option<ScatterResult> {
        match self {
            Self::Lambertian(lambert) => lambert.scatter(ray, ray_hit, rng),
            Self::Metal(metal) => metal.scatter(ray, ray_hit, rng),
            Self::Dielectric(dielectric) => dielectric.scatter(ray, ray_hit, rng),
            Self::Emissive(_) => None,
        }
    }

    pub fn emitted(&self) -> Vec3 {
        match self {
            Self::Lambertian(_) => Vec3::ZERO,
            Self::Metal(_) => Vec3::ZERO,
            Self::Dielectric(_) => Vec3::ZERO,
            Self::Emissive(emissive) => emissive.color,
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
