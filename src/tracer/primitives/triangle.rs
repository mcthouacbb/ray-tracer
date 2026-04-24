use crate::{
    math::Vec3,
    tracer::{
        aabb::AABB,
        primitives::Primitive,
        ray::{Ray, RayHit},
        scene::InstanceId,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    vertices: [Vec3; 3],
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            vertices: [a, b, c],
        }
    }

    pub fn vertices(&self) -> &[Vec3; 3] {
        &self.vertices
    }

    pub fn flat_normal(&self) -> Vec3 {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        edge1.cross(&edge2).normalized()
    }
}

impl Primitive for Triangle {
    fn hit(&self, ray: &Ray, instance_id: InstanceId, primitive_id: u32) -> RayHit {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        let h = ray.dir().cross(&edge2);
        let a = edge1.dot(&h);
        if a.abs() < 0.0001 {
            return RayHit::NONE;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.vertices[0];
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return RayHit::NONE;
        }

        let q = s.cross(&edge1);
        let v = f * ray.dir().dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return RayHit::NONE;
        }

        let t = f * edge2.dot(&q);

        if t > 0.0 {
            RayHit::new(t, instance_id, primitive_id /*, (u, v)*/)
        } else {
            RayHit::NONE
        }
    }

    fn center(&self) -> Vec3 {
        self.bounding_box().center()
    }

    fn bounding_box(&self) -> AABB {
        let mut aabb = AABB::new(self.vertices[0], self.vertices[0]);
        aabb.add_point(self.vertices[1]);
        aabb.add_point(self.vertices[2]);
        aabb
    }

    fn get_normal(&self, _ray: &Ray, _t: f32) -> Vec3 {
        self.flat_normal()
    }
}
