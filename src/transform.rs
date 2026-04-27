use crate::math::{Mat4, Quat, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl Transform {
    pub fn new(position: &Vec3, rotation: &Quat, scale: &Vec3) -> Self {
        Self {
            position: *position,
            rotation: *rotation,
            scale: *scale,
        }
    }

    pub fn look_at(from: &Vec3, at: &Vec3, up: &Vec3) -> Self {
        let rotation_mat = Mat4::look_at(&Vec3::ZERO, &(*at - *from), &up);

        Self::new(
            from,
            &Quat::from_matrix(rotation_mat),
            &Vec3::from_value(1.0),
        )
    }

    pub fn look_at_scale(from: &Vec3, at: &Vec3, up: &Vec3, scale: &Vec3) -> Self {
        let mut result = Self::look_at(from, at, up);
        result.scale = *scale;
        result
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    pub fn position_mut(&mut self) -> &mut Vec3 {
        &mut self.position
    }

    pub fn rotation_mut(&mut self) -> &mut Quat {
        &mut self.rotation
    }

    pub fn scale_mut(&mut self) -> &mut Vec3 {
        &mut self.scale
    }

    pub fn transform(&self) -> Mat4 {
        Mat4::translate(&self.position) * Mat4::rotate(&self.rotation) * Mat4::scale(&self.scale)
    }

    pub fn transform_inv(&self) -> Mat4 {
        Mat4::scale(&self.scale.inverse())
            * Mat4::rotate(&self.rotation).transpose()
            * Mat4::translate(&-self.position)
    }

    pub fn normal_mat(&self) -> Mat4 {
        Mat4::rotate(&self.rotation) * Mat4::scale(&self.scale.inverse())
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(&Vec3::ZERO, &Quat::IDENTITY, &Vec3::from_value(1.0))
    }
}
