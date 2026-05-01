use rand::RngExt;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    elems: [f32; 2],
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { elems: [x, y] }
    }

    pub const fn from_value(v: f32) -> Self {
        Self { elems: [v, v] }
    }

    pub fn x(&self) -> f32 {
        self.elems[0]
    }

    pub fn y(&self) -> f32 {
        self.elems[1]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.elems[0]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.elems[1]
    }

    pub fn sqr_len(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2)
    }

    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.len()
    }

    pub fn inverse(&self) -> Self {
        Self::new(1.0 / self.x(), 1.0 / self.y())
    }

    pub fn min(&self, b: &Self) -> Self {
        Self::new(self.x().min(b.x()), self.y().min(b.y()))
    }

    pub fn max(&self, b: &Self) -> Self {
        Self::new(self.x().max(b.x()), self.y().max(b.y()))
    }

    pub fn dot(&self, b: &Self) -> f32 {
        self.x() * b.x() + self.y() * b.y()
    }

    pub fn pairwise(&self, other: &Self) -> Self {
        Self::new(self.x() * other.x(), self.y() * other.y())
    }

    pub fn random_range(min: f32, max: f32, rng: &mut impl RngExt) -> Self {
        Self::new(rng.random_range(min..=max), rng.random_range(min..=max))
    }

    pub fn random_unit(rng: &mut impl RngExt) -> Self {
        loop {
            let v = Self::new(rng.random_range(-1.0..=1.0), rng.random_range(-1.0..=1.0));
            let sqr_len = v.sqr_len();
            if 1e-20 <= sqr_len && sqr_len <= 1.0 {
                return v / sqr_len.sqrt();
            }
        }
    }
}

impl ops::Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl ops::IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..2 {
            self.elems[i] += rhs.elems[i]
        }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..2 {
            self.elems[i] -= rhs.elems[i]
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..2 {
            self.elems[i] *= rhs;
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        let mut result = rhs;
        result *= self;
        result
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        for i in 0..2 {
            self.elems[i] /= rhs;
        }
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y())
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

    use crate::math::Vec2;

    #[test]
    fn test_get() {
        let mut a = Vec2::new(-1837.2827, 681828.3434);
        assert_eq!(a.x(), -1837.2827);
        assert_eq!(a.y(), 681828.3434);

        *a.x_mut() = -771112.4321;
        *a.y_mut() = 28374.3827;

        assert_eq!(a.x(), -771112.4321);
        assert_eq!(a.y(), 28374.3827);

        assert_eq!(a.x(), a[0]);
        assert_eq!(a.y(), a[1]);
    }

    #[test]
    fn test_len() {
        let a = Vec2::new(1.0, 1.0);
        assert_eq!(a.sqr_len(), 2.0);
        assert_float_absolute_eq!(a.len(), 2.0f32.sqrt());
        assert_float_absolute_eq!(a.normalized().sqr_len(), 1.0);

        let a = Vec2::new(28.0, -35.5);
        assert_eq!(a.sqr_len(), 2044.25);
        assert_float_absolute_eq!(a.len(), 2044.25f32.sqrt());

        assert_eq!((2.0 * a).sqr_len(), 8177.0);
        assert_float_absolute_eq!((2.0 * a).len(), 8177.0f32.sqrt());

        assert_eq!((a / 2.0).sqr_len(), 511.0625);
        assert_float_absolute_eq!((a / 2.0).len(), 511.0625f32.sqrt());

        assert_eq!(a.dot(&a), a.sqr_len());
        assert_float_absolute_eq!(a.normalized().sqr_len(), 1.0);
    }

    #[test]
    fn test_min_max() {
        let a = Vec2::new(25.0, -35.0);
        let b = Vec2::new(-5.0, 15.0);

        assert_eq!(a.min(&b), Vec2::new(-5.0, -35.0));
        assert_eq!(a.max(&b), Vec2::new(25.0, 15.0));

        assert_eq!(a.min(&b) + a.max(&b), a + b);
    }

    #[test]
    fn test_add_sub() {
        let a = Vec2::new(719383.91637, -575633.28347);
        let b = Vec2::new(19439.2834, 8174.3838);

        assert_eq!(
            a + b,
            Vec2::new(719383.91637 + 19439.2834, -575633.28347 + 8174.3838)
        );

        assert_eq!(
            a - b,
            Vec2::new(719383.91637 - 19439.2834, -575633.28347 - 8174.3838)
        );

        assert_eq!(-a, Vec2::new(-719383.91637, 575633.28347));

        assert_eq!(-(a - b), b - a);
    }

    #[test]
    fn test_mul_div() {
        let a = Vec2::new(91738.3847, 483721.2268);
        for i in -100..=100 {
            let d = i as f32 / 50.0;
            assert_eq!(a / d, Vec2::new(91738.3847 / d, 483721.2268 / d));
            assert_eq!(a * d, Vec2::new(91738.3847 * d, 483721.2268 * d));
        }
    }
}
