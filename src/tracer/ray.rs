use crate::{math::Vec3, tracer::sphere::Sphere};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    t: f32,
    normal: Vec3,
}

impl RayHit {
    const NONE: Self = Self {
        t: f32::INFINITY,
        normal: Vec3::ZERO,
    };

    pub fn new(t: f32, normal: Vec3) -> Self {
        Self { t, normal }
    }

    pub fn replace_if_closer(&mut self, hit: &Self) {
        if hit.t < self.t {
            *self = *hit;
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}

// return value is temporary
pub fn trace_sphere(sphere: &Sphere, ray: &Ray) -> RayHit {
    let oc = *sphere.center() - *ray.origin();
    let a = ray.dir().dot(ray.dir());
    let b = -2.0 * ray.dir().dot(&oc);
    let c = oc.dot(&oc) - sphere.radius().powi(2);
    let discriminant = b * b - 4.0 * a * c;
    if discriminant >= 0.0 {
        let t = -(b + discriminant.sqrt()) / (2.0 * a);
        let pos = ray.origin + ray.dir * t;
        RayHit::new(t, (pos - *sphere.center()) / sphere.radius())
    } else {
        RayHit::NONE
    }
}
