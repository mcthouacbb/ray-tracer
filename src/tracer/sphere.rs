use crate::{
    math::Vec3,
    tracer::{
        hittable::Hittable,
        material::Material,
        ray::{Ray, RayHit},
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: &Material) -> Self {
        Self {
            center,
            radius,
            material: *material,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn trace(&self, ray: &Ray) -> RayHit {
        let oc = self.center() - ray.origin();
        let a = ray.dir().dot(&ray.dir());
        let b = -2.0 * ray.dir().dot(&oc);
        let c = oc.dot(&oc) - self.radius().powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let dist = if t1 > 0.0 {
                t1
            } else if t2 > 0.0 {
                t2
            } else {
                return RayHit::NONE;
            };
            let pos = ray.origin() + ray.dir() * dist;
            RayHit::new(dist, (pos - self.center()) / self.radius(), self.material)
        } else {
            RayHit::NONE
        }
    }
}
