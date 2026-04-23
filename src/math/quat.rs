use std::{f32, ops};

use rand::RngExt;

use crate::math::{Mat4, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Quat {
    elems: [f32; 4],
}

impl Quat {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            elems: [x, y, z, w],
        }
    }

    // https://github.com/mrdoob/three.js/blob/dev/src/math/Quaternion.js#L405
    pub fn from_matrix(mat: Mat4) -> Self {
        let trace = mat[0][0] + mat[1][1] + mat[2][2];
        if trace > 0.0 {
            let s = 2.0 * (trace + 1.0).sqrt();
            Self::new(
                (mat[1][2] - mat[2][1]) / s,
                (mat[2][0] - mat[0][2]) / s,
                (mat[0][1] - mat[1][0]) / s,
                0.25 * s,
            )
        } else if mat[0][0] > mat[1][1] && mat[0][0] > mat[2][2] {
            let s = 2.0 * (1.0 + mat[0][0] - mat[1][1] - mat[2][2]).sqrt();
            Self::new(
                0.25 * s,
                (mat[1][0] + mat[0][1]) / s,
                (mat[2][0] + mat[0][2]) / s,
                (mat[1][2] - mat[2][1]) / s,
            )
        } else if mat[1][1] > mat[2][2] {
            let s = 2.0 * (1.0 + mat[1][1] - mat[0][0] - mat[2][2]).sqrt();
            Self::new(
                (mat[1][0] + mat[0][1]) / s,
                0.25 * s,
                (mat[2][1] + mat[1][2]) / s,
                (mat[2][0] - mat[0][2]) / s,
            )
        } else {
            let s = 2.0 * (1.0 + mat[2][2] - mat[1][1] - mat[0][0]);
            Self::new(
                (mat[2][0] + mat[0][2]) / s,
                (mat[2][1] + mat[1][2]) / s,
                0.25 * s,
                (mat[0][1] - mat[1][0]) / s,
            )
        }
    }

    pub fn x(&self) -> f32 {
        self.elems[0]
    }

    pub fn y(&self) -> f32 {
        self.elems[1]
    }

    pub fn z(&self) -> f32 {
        self.elems[2]
    }

    pub fn w(&self) -> f32 {
        self.elems[3]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.elems[0]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.elems[1]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.elems[2]
    }

    pub fn w_mut(&mut self) -> &mut f32 {
        &mut self.elems[3]
    }

    pub fn sqr_len(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2) + self.w().powi(2)
    }

    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    pub fn from_axis_angle(axis: &Vec3, angle: f32) -> Self {
        let cos = (angle / 2.0).cos();
        let sin = (angle / 2.0).sin();
        Self::new(sin * axis.x(), sin * axis.y(), sin * axis.z(), cos)
    }

    pub fn rotate_x(angle: f32) -> Self {
        Self::new((angle / 2.0).sin(), 0.0, 0.0, (angle / 2.0).cos())
    }

    pub fn rotate_y(angle: f32) -> Self {
        Self::new(0.0, (angle / 2.0).sin(), 0.0, (angle / 2.0).cos())
    }

    pub fn rotate_z(angle: f32) -> Self {
        Self::new(0.0, 0.0, (angle / 2.0).sin(), (angle / 2.0).cos())
    }

    pub fn from_euler_zxy(angles: &Vec3) -> Self {
        let s1 = (angles.x() / 2.0).sin();
        let s2 = (angles.y() / 2.0).sin();
        let s3 = (angles.z() / 2.0).sin();

        let c1 = (angles.x() / 2.0).cos();
        let c2 = (angles.y() / 2.0).cos();
        let c3 = (angles.z() / 2.0).cos();

        Self::new(
            s1 * c2 * c3 + c1 * s2 * s3,
            c1 * s2 * c3 - s1 * c2 * s3,
            c1 * c2 * s3 - s1 * s2 * c3,
            c1 * c2 * c3 + s1 * s2 * s3,
        )
    }

    pub fn random(rng: &mut impl RngExt) -> Self {
        let u = rng.random_range(0.0f32..=1.0f32);
        let v = rng.random_range(0.0f32..=1.0f32);
        let w = rng.random_range(0.0f32..=1.0f32);
        let s = (1.0 - u).sqrt();
        let s2 = u.sqrt();
        Self::new(
            s * (f32::consts::TAU * v).sin(),
            s * (f32::consts::TAU * v).cos(),
            s2 * (f32::consts::TAU * w).sin(),
            s2 * (f32::consts::TAU * w).cos(),
        )
    }
}

impl ops::Index<usize> for Quat {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl ops::IndexMut<usize> for Quat {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}

impl ops::Mul<Quat> for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Quat::new(
            self.x() * rhs.w() + self.w() * rhs.x() + self.y() * rhs.z() - self.z() * rhs.y(),
            self.y() * rhs.w() + self.w() * rhs.y() + self.z() * rhs.x() - self.x() * rhs.z(),
            self.z() * rhs.w() + self.w() * rhs.z() + self.x() * rhs.y() - self.y() * rhs.x(),
            self.w() * rhs.w() - self.x() * rhs.x() - self.y() * rhs.y() - self.z() * rhs.z(),
        )
    }
}

impl ops::MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
