use std::mem::swap;
use std::f32::consts::PI;

pub type F = f32;
pub type I = i32;
pub type UI = u32;
pub type S = usize;

pub const EPSILON: F = 1e-10;

pub const X_AXIS: UI = 0;
pub const Y_AXIS: UI = 1;
pub const Z_AXIS: UI = 2;

pub fn lerp(t: F, v1: F, v2: F) -> F {
    (1.0 - t) * v1 + t * v2
}

pub fn quadratic(a: F, b: F, c: F) -> Option<(F, F)> {
    let discrim = b * b - 4.0 * a * c;
    if discrim < 0.0 { return None }
    let sqrt_d = discrim.sqrt();
    let q;
    if b < 0.0 { q = -0.5 * (b - sqrt_d) }
    else       { q = -0.5 * (b + sqrt_d) }
    let mut t0 = q / a;
    let mut t1 = c / q;
    if t0 > t1 { swap(&mut t0, &mut t1); }
    Some((t0, t1))
}

// pub fn quadratic(a: F, b: F, c: F) -> Option<(F, F)> {
//     let half_b = b / 2.0;
//     let discrim = half_b * half_b - a * c;
//     if discrim < 0.0 { return None }
//     let sqrtd = discrim.sqrt();
//     let root1 = (-half_b - sqrtd) / a;
//     let root2 = (-half_b + sqrtd) / a;
//     return Some((root1, root2));
// }

pub fn linear_system(a: [[F; 2]; 2], b: [F; 2]) -> (Option<F>, Option<F>) {
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    if det.abs() < 1e-10 { return (None, None) }
    let x0 = (a[1][1] * b[0] - a[0][1] * b[1]) / det;
    let x1 = (a[0][0] * b[1] - a[1][0] * b[0]) / det;
    let out0 = match x0.is_nan() {
        true => None,
        false => Some(x0),
    };
    let out1 = match x1.is_nan() {
        true => None,
        false => Some(x1),
    };
    (out0, out1)
}

pub fn gamma(n: F) -> F {
    (n * EPSILON) / (1.0 - n * EPSILON)
}

pub fn deg2rad(deg: F) -> F {
    deg / 180.0 * PI
}

pub fn rad2deg(rad: F) -> F {
    rad * 180.0 / PI
}