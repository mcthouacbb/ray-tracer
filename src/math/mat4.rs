use std::ops::{Index, IndexMut, Mul};

use crate::math::{Vec3, Vec4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    columns: [Vec4; 4],
}

impl Mat4 {
    const ZERO: Self = Self::from_elems(&[0.0; 16]);
    const IDENTITY: Self = Self::from_elems(&[
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]);

    pub const fn from_elems(elems: &[f32]) -> Self {
        assert!(elems.len() >= 16);
        Self {
            columns: [
                Vec4::new(elems[0], elems[1], elems[2], elems[3]),
                Vec4::new(elems[4], elems[5], elems[6], elems[7]),
                Vec4::new(elems[8], elems[9], elems[10], elems[11]),
                Vec4::new(elems[12], elems[13], elems[14], elems[15]),
            ],
        }
    }

    pub fn from_cols(columns: [Vec4; 4]) -> Self {
        Self { columns }
    }

    pub fn index_raw(&self, idx: usize) -> f32 {
        self[idx / 4][idx % 4]
    }

    pub fn index_raw_mut(&mut self, idx: usize) -> &mut f32 {
        &mut self[idx / 4][idx % 4]
    }

    pub fn rows(&self) -> [Vec4; 4] {
        [
            Vec4::new(self[0][0], self[1][0], self[2][0], self[3][0]),
            Vec4::new(self[0][1], self[1][1], self[2][1], self[3][1]),
            Vec4::new(self[0][2], self[1][2], self[2][2], self[3][2]),
            Vec4::new(self[0][3], self[1][3], self[2][3], self[3][3]),
        ]
    }

    pub fn transpose(&self) -> Self {
        Self::from_cols(self.rows())
    }

    pub fn translate(t: Vec3) -> Self {
        let mut result = Self::IDENTITY;
        result[3][0] = t[0];
        result[3][1] = t[1];
        result[3][2] = t[2];
        result
    }

    pub fn rotate_x(angle: f32) -> Self {
        let mut result = Self::IDENTITY;
        let cos = angle.cos();
        let sin = angle.sin();
        result[1][1] = cos;
        result[1][2] = sin;
        result[2][1] = -sin;
        result[2][2] = cos;
        result
    }

    pub fn rotate_y(angle: f32) -> Self {
        let mut result = Self::IDENTITY;
        let cos = angle.cos();
        let sin = angle.sin();
        result[2][2] = cos;
        result[2][0] = sin;
        result[0][2] = -sin;
        result[0][0] = cos;
        result
    }

    pub fn rotate_z(angle: f32) -> Self {
        let mut result = Self::IDENTITY;
        let cos = angle.cos();
        let sin = angle.sin();
        result[0][0] = cos;
        result[0][1] = sin;
        result[1][0] = -sin;
        result[1][1] = cos;
        result
    }

    pub fn scale(s: Vec3) -> Self {
        let mut result = Self::IDENTITY;
        result[0][0] = s[0];
        result[1][1] = s[1];
        result[2][2] = s[2];
        result
    }

    pub fn mul_vec(&self, vec: Vec4) -> Vec4 {
        let rows = self.rows();
        Vec4::new(
            rows[0].dot(&vec),
            rows[1].dot(&vec),
            rows[2].dot(&vec),
            rows[3].dot(&vec),
        )
    }

    pub fn mul_mat(&self, mat: Self) -> Self {
        Self::from_cols([
            self.mul_vec(mat[0]),
            self.mul_vec(mat[1]),
            self.mul_vec(mat[2]),
            self.mul_vec(mat[3]),
        ])
    }
}

impl Index<usize> for Mat4 {
    type Output = Vec4;
    fn index(&self, index: usize) -> &Self::Output {
        &self.columns[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.columns[index]
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self.mul_vec(rhs)
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_mat(rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Mat4, Vec4};
    use rand::{RngExt, SeedableRng, rngs::Xoshiro256PlusPlus};

    fn gen_rand(rng: &mut impl RngExt, n: usize) -> Vec<f32> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(rng.random_range(-1.0..=1.0));
        }
        result
    }

    #[test]
    fn test_get() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(551992);

        let rand = gen_rand(&mut rng, 16);
        let mat = Mat4::from_elems(&rand);

        for c in 0..4 {
            for r in 0..4 {
                assert_eq!(mat[c][r], rand[4 * c + r]);
                assert_eq!(mat.index_raw(4 * c + r), rand[4 * c + r]);
            }
        }

        let rows = mat.rows();
        let transpose = mat.transpose();
        for c in 0..4 {
            for r in 0..4 {
                assert_eq!(rows[r][c], rand[4 * c + r]);
                assert_eq!(transpose[r][c], rand[4 * c + r]);
            }
        }
    }

    fn test_mul() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(2918237);
        let rand = gen_rand(&mut rng, 4);

        let vec = Vec4::new(rand[0], rand[1], rand[2], rand[3]);

        assert_eq!(vec, Mat4::IDENTITY * vec);
        assert_eq!(vec, Mat4::IDENTITY.mul_vec(vec));

        assert_eq!(Mat4::IDENTITY, Mat4::IDENTITY.mul_mat(Mat4::IDENTITY));

        let mut mat = Mat4::IDENTITY;
        mat[2][2] = 0.0;

        assert_eq!(Vec4::new(rand[0], rand[1], 0.0, rand[3]), mat.mul_vec(vec));

        mat[0][1] = 2.5;
        mat[1][0] = 0.8;
        mat[3][3] = 4.3;
        assert_eq!(
            Vec4::new(
                rand[0] + rand[1] * 0.8,
                rand[0] * 2.5 + rand[1],
                0.0,
                rand[3] * 4.3
            ),
            mat.mul_vec(vec)
        );
    }
}
