use crate::math::Vec3;

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
}
