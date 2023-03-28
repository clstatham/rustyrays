use crate::common::*;
use crate::vector::*;

pub type Color3 = Vec3;

pub fn color3(r: F, g: F, b: F) -> Color3 {
    vec3(r, g, b)
}
pub fn black() -> Color3 {
    color3(0.0, 0.0, 0.0)
}

pub fn color_to_pixel(col: Color3, gamma: F) -> [u8; 4] {
    [
        (col.x.powf(gamma).clamp(0.0, 0.9999) * 255.0) as u8,
        (col.y.powf(gamma).clamp(0.0, 0.9999) * 255.0) as u8,
        (col.z.powf(gamma).clamp(0.0, 0.9999) * 255.0) as u8,
        255,
    ]
}

pub fn blackbody(lambda: &[F], t: F) -> Vec<F> {
    let c = 299792458 as F;
    let h = 6.626_069_7e-34;
    let kb = 1.3806488e-23;
    let mut le = vec![];
    for val in lambda {
        let l = val * 1e-9;
        let lambda5 = l * l * l * l * l;
        le.push((2.0 * h * c * c) / (lambda5 * (F::exp((h * c) / (l * kb * t)) - 1.0)));
    }

    le
}

pub fn blackbody_normalized(lambda: &[F], t: F) -> Vec<F> {
    let le = blackbody(lambda, t);
    let lambda_max = 2.897_772e-3 / t * 1e9;
    let max_l = blackbody(&[lambda_max], t)[0];
    le.iter().map(|i| i / max_l).collect()
}
