use crate::{
    math::Vec3,
    tracer::ray::{Ray, RayHit},
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    // return value is temporary
    pub fn trace(&self, ray: &Ray) -> RayHit {
        let oc = *self.center() - *ray.origin();
        let a = ray.dir().dot(ray.dir());
        let b = -2.0 * ray.dir().dot(&oc);
        let c = oc.dot(&oc) - self.radius().powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t = -(b + discriminant.sqrt()) / (2.0 * a);
            let pos = *ray.origin() + *ray.dir() * t;
            RayHit::new(t, (pos - *self.center()) / self.radius())
        } else {
            RayHit::NONE
        }
    }
}
