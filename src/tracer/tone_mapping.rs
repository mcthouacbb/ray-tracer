use crate::math::{Mat4, Vec3, Vec4};

const ACES_INPUT_MAT: Mat4 = Mat4::from_cols([
    Vec4::new(0.59719, 0.07600, 0.02840, 0.0),
    Vec4::new(0.35458, 0.90834, 0.13383, 0.0),
    Vec4::new(0.04823, 0.01566, 0.83777, 0.0),
    Vec4::new(0.0, 0.0, 0.0, 1.0),
]);

const ACES_OUTPUT_MAT: Mat4 = Mat4::from_cols([
    Vec4::new(1.60475, -0.10208, -0.00327, 0.0),
    Vec4::new(-0.53108, 1.10813, -0.07276, 0.0),
    Vec4::new(-0.07367, -0.00605, 1.07602, 0.0),
    Vec4::new(0.0, 0.0, 0.0, 1.0),
]);

fn rtt_odt_fit(v: &Vec3) -> Vec3 {
    let a = v.pairwise(&(*v + Vec3::from_value(0.0245786))) - Vec3::from_value(0.000090537);
    let b = v.pairwise(&(0.983729 * *v + Vec3::from_value(0.4329510))) + Vec3::from_value(0.238081);
    a.pairwise(&b.inverse())
}

pub fn aces(color: &Vec3) -> Vec3 {
    let v = ACES_INPUT_MAT.transform_dir(color);
    let v = rtt_odt_fit(&v);
    ACES_OUTPUT_MAT.transform_dir(&v)
}
