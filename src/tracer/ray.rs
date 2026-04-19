use crate::{math::Vec3, tracer::sphere::Sphere};

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
}

// return value is temporary
pub fn trace_sphere(sphere: &Sphere, ray: &Ray) -> Option<()> {
    let oc = *sphere.center() - *ray.origin();
    let a = ray.dir().dot(ray.dir());
    let b = -2.0 * ray.dir().dot(&oc);
    let c = oc.dot(&oc) - sphere.radius().powi(2);
    if b * b - 4.0 * a * c >= 0.0 {
        Some(())
    } else {
        None
    }
}
