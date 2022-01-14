use crate::common::*;
use crate::vector::*;

pub type Color3 = Vec3;

pub fn color3(r: F, g: F, b: F) -> Color3 { vec3(r, g, b) }
pub fn black() -> Color3 { color3(0.0, 0.0, 0.0) }

pub fn color_to_pixel(col: Color3) -> [u8; 4] {
    [
        (col.x.sqrt().clamp(0.0, 0.9999) * 255.0) as u8,
        (col.y.sqrt().clamp(0.0, 0.9999) * 255.0) as u8,
        (col.z.sqrt().clamp(0.0, 0.9999) * 255.0) as u8,
        255,
    ]
}
