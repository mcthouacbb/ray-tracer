use std::ops::{self, Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    elems: [f32; 3],
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { elems: [x, y, z] }
    }

    pub fn from_value(v: f32) -> Self {
        Self { elems: [v, v, v] }
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

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.elems[0]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.elems[1]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.elems[2]
    }

    pub fn sqr_len(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
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
        )
    }

    pub fn max(&self, b: &Self) -> Self {
        Self::new(
            self.x().max(b.x()),
            self.y().max(b.y()),
            self.z().max(b.z()),
        )
    }

    pub fn dot(&self, b: &Self) -> f32 {
        self.x() * b.x() + self.y() * b.y() + self.z() * b.z()
    }

    pub fn cross(&self, b: &Self) -> Vec3 {
        return Vec3::new(
            self.y() * b.z() - self.z() * b.y(),
            self.z() * b.x() - self.x() * b.z(),
            self.x() * b.y() - self.y() * b.x(),
        );
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.elems[i] += rhs.elems[i]
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.elems[i] -= rhs.elems[i]
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..3 {
            self.elems[i] *= rhs;
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut result = rhs;
        result *= self;
        result
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        for i in 0..3 {
            self.elems[i] /= rhs;
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

    use crate::math::Vec3;

    #[test]
    fn test_get() {
        let mut a = Vec3::new(3248.24873, 1034994.01992, -302837.4929);
        assert_eq!(a.x(), 3248.24873);
        assert_eq!(a.y(), 1034994.01992);
        assert_eq!(a.z(), -302837.4929);

        *a.x_mut() = 991112.1234;
        *a.y_mut() = -2483727.7777;
        *a.z_mut() = 8374722.11554;

        assert_eq!(a.x(), 991112.1234);
        assert_eq!(a.y(), -2483727.7777);
        assert_eq!(a.z(), 8374722.11554);

        assert_eq!(a.x(), a[0]);
        assert_eq!(a.y(), a[1]);
        assert_eq!(a.z(), a[2]);
    }

    #[test]
    fn test_len() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(a.sqr_len(), 3.0);
        assert_float_absolute_eq!(a.len(), 3.0f32.sqrt());
        assert_float_absolute_eq!(a.normalized().sqr_len(), 1.0);

        let a = Vec3::new(25.5, -48.0, 10.5);
        assert_eq!(a.sqr_len(), 3064.5);
        assert_float_absolute_eq!(a.len(), 3064.5f32.sqrt());

        assert_eq!((2.0 * a).sqr_len(), 12258.0);
        assert_float_absolute_eq!((2.0 * a).len(), 12258.0f32.sqrt());

        assert_eq!((a / 2.0).sqr_len(), 766.125);
        assert_float_absolute_eq!((a / 2.0).len(), 766.125f32.sqrt());

        assert_eq!(a.dot(&a), a.sqr_len());
        assert_float_absolute_eq!(a.normalized().sqr_len(), 1.0);
    }

    #[test]
    fn test_min_max() {
        let a = Vec3::new(25.0, -35.0, 14.0);
        let b = Vec3::new(-5.0, 15.0, 10.0);

        assert_eq!(a.min(&b), Vec3::new(-5.0, -35.0, 10.0));
        assert_eq!(a.max(&b), Vec3::new(25.0, 15.0, 14.0));

        assert_eq!(a.min(&b) + a.max(&b), a + b);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(15.0, -32.0, 17.0);
        let b = Vec3::new(-5.0, 15.0, 10.0);
        let c = a.cross(&b);
        let d = b.cross(&a);

        assert_float_absolute_eq!(c.dot(&a), 0.0);
        assert_float_absolute_eq!(c.dot(&b), 0.0);
        assert_float_absolute_eq!(d.dot(&a), 0.0);
        assert_float_absolute_eq!(d.dot(&b), 0.0);

        assert_float_absolute_eq!((d + c).sqr_len(), 0.0);

        assert_eq!(a.cross(&a), Vec3::ZERO);
        assert_eq!(b.cross(&b), Vec3::ZERO);
    }

    #[test]
    fn test_add_sub() {
        let a = Vec3::new(28374.28, -575633.28347, 719383.91637);
        let b = Vec3::new(18374.2871, 19439.2834, 8174.3838);

        assert_eq!(
            a + b,
            Vec3::new(
                28374.28 + 18374.2871,
                -575633.28347 + 19439.2834,
                719383.91637 + 8174.3838
            )
        );

        assert_eq!(
            a - b,
            Vec3::new(
                28374.28 - 18374.2871,
                -575633.28347 - 19439.2834,
                719383.91637 - 8174.3838
            )
        );

        assert_eq!(-a, Vec3::new(-28374.28, 575633.28347, -719383.91637));

        assert_eq!(-(a - b), b - a);
    }

    #[test]
    fn test_mul_div() {
        let a = Vec3::new(91738.3847, 76384.2847, -81827.22);
        for i in -100..=100 {
            let d = i as f32 / 50.0;
            assert_eq!(
                a / d,
                Vec3::new(91738.3847 / d, 76384.2847 / d, -81827.22 / d)
            );
            assert_eq!(
                a * d,
                Vec3::new(91738.3847 * d, 76384.2847 * d, -81827.22 * d)
            );
        }
    }
}
