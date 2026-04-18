use std::ops::{self, Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    elems: [f32; 4],
}

impl Vec4 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            elems: [x, y, z, w],
        }
    }

    pub fn from_value(v: f32) -> Self {
        Self {
            elems: [v, v, v, v],
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
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w()
    }

    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.len()
    }

    pub fn min(&self, b: &Self) -> Self {
        Self::new(
            self.x().min(b.x()),
            self.y().min(b.y()),
            self.z().min(b.z()),
            self.w().min(b.w()),
        )
    }

    pub fn max(&self, b: &Self) -> Self {
        Self::new(
            self.x().max(b.x()),
            self.y().max(b.y()),
            self.z().max(b.z()),
            self.w().max(b.w()),
        )
    }

    pub fn dot(&self, b: &Self) -> f32 {
        self.x() * b.x() + self.y() * b.y() + self.z() * b.z() + self.w() * b.w()
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}

impl ops::AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            self.elems[i] += rhs.elems[i]
        }
    }
}

impl ops::Add<Vec4> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl ops::SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            self.elems[i] += rhs.elems[i]
        }
    }
}

impl ops::Sub<Vec4> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl ops::MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..4 {
            self.elems[i] *= rhs;
        }
    }
}

impl ops::Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut result = rhs;
        result *= self;
        result
    }
}

impl ops::DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        for i in 0..4 {
            self.elems[i] /= rhs;
        }
    }
}

impl ops::Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl ops::Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

    use crate::math::Vec4;

    #[test]
    fn test_get() {
        let mut a = Vec4::new(0.3444221, 27.3847283, -8119.7491, 12838474.3);
        assert_eq!(a.x(), 0.3444221);
        assert_eq!(a.y(), 27.3847283);
        assert_eq!(a.z(), -8119.7491);
        assert_eq!(a.w(), 12838474.3);
        *a.x_mut() = 991112.1234;
        *a.y_mut() = -2483727.7777;
        *a.z_mut() = 8374722.11554;
        *a.w_mut() = -1.1552443;

        assert_eq!(a.x(), 991112.1234);
        assert_eq!(a.y(), -2483727.7777);
        assert_eq!(a.z(), 8374722.11554);
        assert_eq!(a.w(), -1.1552443);

        assert_eq!(a.x(), a[0]);
        assert_eq!(a.y(), a[1]);
        assert_eq!(a.z(), a[2]);
        assert_eq!(a.w(), a[3]);
    }

    #[test]
    fn test_len() {
        let a = Vec4::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(a.sqr_len(), 4.0);
        assert_float_absolute_eq!(a.len(), 2.0f32);
        assert_float_absolute_eq!(a.normalized().sqr_len(), 1.0);

        let a = Vec4::new(-28.5, 52.0, -21.5, 14.0);
        assert_eq!(a.sqr_len(), 4174.5);
        assert_float_absolute_eq!(a.len(), 4174.5f32.sqrt());

        assert_eq!((2.0 * a).sqr_len(), 16698.0);
        assert_float_absolute_eq!((2.0 * a).len(), 16698.0f32.sqrt());

        assert_eq!((a / 2.0).sqr_len(), 1043.625);
        assert_float_absolute_eq!((a / 2.0).len(), 1043.625f32.sqrt());

        assert_eq!(a.dot(&a), a.sqr_len());
        assert_float_absolute_eq!(a.normalized().sqr_len(), 1.0);
    }

    #[test]
    fn test_min_max() {
        let a = Vec4::new(25.0, -35.0, 14.0, 10394.183);
        let b = Vec4::new(-5.0, 15.0, 10.0, -2244.532);

        assert_eq!(a.min(&b), Vec4::new(-5.0, -35.0, 10.0, -2244.532));
        assert_eq!(a.max(&b), Vec4::new(25.0, 15.0, 14.0, 10394.183));

        assert_eq!(a.min(&b) + a.max(&b), a + b);
    }

    fn test_add_sub() {
        let a = Vec4::new(28374.28, -575633.28347, 719383.91637, 22344.1552);
        let b = Vec4::new(18374.2871, 19439.2834, 8174.3838, -878734.51);

        assert_eq!(
            a + b,
            Vec4::new(
                28374.28 + 18374.2871,
                -575633.28347 + 19439.2834,
                719383.91637 + 8174.3838,
                22344.1552 + -878734.51
            )
        );

        assert_eq!(
            a - b,
            Vec4::new(
                28374.28 - 18374.2871,
                -575633.28347 - 19439.2834,
                719383.91637 - 8174.3838,
                22344.1552 - -878734.51
            )
        );

        assert_eq!(
            -a,
            Vec4::new(-28374.28, 575633.28347, -719383.91637, -22344.1552)
        );

        assert_eq!(-(a - b), b - a);
    }

    fn test_mul_div() {
        let a = Vec4::new(91738.3847, 76384.2847, -81827.22, 283847.11331);
        for i in -100..=100 {
            let d = i as f32 / 50.0;
            assert_eq!(
                a / d,
                Vec4::new(
                    91738.3847 / d,
                    76384.2847 / d,
                    -81827.22 / d,
                    283847.11331 / d
                )
            );
            assert_eq!(
                a * d,
                Vec4::new(
                    91738.3847 * d,
                    76384.2847 * d,
                    -81827.22 * d,
                    283847.11331 * d
                )
            );
        }
    }
}
