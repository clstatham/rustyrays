use std::f64::consts::PI;

use crate::common::F;

extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
pub type Vec3 = na::Vector3<F>;
pub type Point3 = na::Vector3<F>;
pub type Normal3 = na::Vector3<F>;
pub type Vec2 = na::Vector2<F>;
pub type Point2 = na::Vector2<F>;
pub type NAPoint3 = na::Point3<F>;
pub type NAPoint2 = na::Point2<F>;

pub fn vec3(x: F, y: F, z: F) -> Vec3 {
    // na::vector![x, y, z]
    Vec3::new(x, y, z)
}
pub fn point3(x: F, y: F, z: F) -> Point3 {
    Point3::new(x, y, z)
}
pub fn normal3(x: F, y: F, z: F) -> Normal3 {
    Normal3::new(x, y, z)
}
pub fn point2(x: F, y: F) -> Point2 {
    Point2::new(x, y)
}
pub fn vec2(x: F, y: F) -> Vec2 {
    Vec2::new(x, y)
}

pub fn distance3d(a: &Point3, b: &Point3) -> F {
    (a - b).magnitude()
}

pub fn distance_squared3d(a: &Point3, b: &Point3) -> F {
    (a - b).magnitude_squared()
}

pub fn spherical_theta(v: &Vec3) -> F {
    F::acos(F::clamp(v.z, -1.0, 1.0))
}

pub fn spherical_phi(v: &Vec3) -> F {
    let p_ = F::atan2(v.y, v.x);
    match p_ {
        p if p < 0.0 => p + 2.0 * PI,
        p => p,
    }
}

// Standard 3d float vector.
// #[derive(Clone, Copy, PartialEq, Debug)]
// pub struct Vec3 {
//     pub x: F,
//     pub y: F,
//     pub z: F,
// }

// #[derive(Clone, Copy, PartialEq, Debug)]
// pub struct Vec2 {
//     pub u: F,
//     pub v: F,
// }

// pub type Vec3 = nalgebra::Vector3<F>;
// pub type Point3 = Vec3;
// pub type Normal3 = Vec3;
// pub type Point2 = Vec2;
// pub type Color3 = Vec3;

// pub fn zero_vec3() -> Vec3 {
//     Vec3::zeros()
// }

// pub fn zero_vec2() -> Vec2 {
//     Vec2::zeros()
// }

// pub fn zero_point3() -> Point3 {
//     Point3::zeros()
// }

// pub fn zero_point2() -> Vec2 {
//     Point2::zeros()
// }

// pub fn zero_normal3() -> Normal3 {
//     Normal3::zeros()
// }

// pub fn black() -> Normal3 {
//     Color3::zeros()
// }

// // impl Default for Vec3 {
// //     fn default() -> Self {
// //         Self {x: 0.0, y: 0.0, z: 0.0}
// //     }
// // }

// // impl Default for Vec2 {
// //     fn default() -> Self {
// //         Self {u: 0.0, v: 0.0}
// //     }
// // }

// pub fn vec3(x: F, y: F, z: F) -> Vec3 {
//     Vec3::new(x, y, z)
// }

// pub fn vec2(u: F, v: F,) -> Vec2 {
//     Vec2::new(u, v)
// }

// pub fn point3(x: F, y: F, z: F) -> Point3 {
//     Point3::new(x, y, z)
// }

// pub fn point2(u: F, v: F,) -> Point2 {
//     Point2::new(u, v)
// }

// pub fn normal3(x: F, y: F, z: F) -> Normal3 {
//     Normal3::new(x, y, z)
// }

// pub fn color3(r: F, g: F, b: F) -> Color3 {
//     Color3::new(r, g, b)
// }

// pub fn color_to_pixel(col: Color3) -> [u8; 4] {
//     [
//         (col.x.sqrt().clamp(0.0, 0.9999) * 255.0) as u8,
//         (col.y.sqrt().clamp(0.0, 0.9999) * 255.0) as u8,
//         (col.z.sqrt().clamp(0.0, 0.9999) * 255.0) as u8,
//         255,
//     ]
// }

// pub fn generate_normal(a: Vec3, b: Vec3) -> Normal3 {
//     a.cross(b)
// }

// pub fn face_forward(n: Normal3, v: Vec3) -> Vec3 {
//     if n.dot(v) < 0.0 { -n } else { n }
// }

// impl Vec3 {
//     pub fn dot(self, other: Vec3) -> F {
//         self.x * other.x + self.y * other.y + self.z * other.z
//     }

//     pub fn absdot(self, other: Vec3) -> F {
//         self.dot(other).abs()
//     }

//     pub fn cross(self, other: Vec3) -> Vec3 {
//         Vec3 {
//             x: self.y*other.z - self.z*other.y,
//             y: self.z*other.x - self.x*other.z,
//             z: self.x*other.y - self.y*other.x
//         }
//     }

//     pub fn length_squared(self) -> F {
//         self.x*self.x + self.y*self.y + self.z*self.z
//     }
//     pub fn length(self) -> F { self.length_squared().sqrt() }

//     pub fn normalize(self) -> Vec3 {
//         let l = self.length();
//         assert_ne!(l, 0.0);
//         self / l
//     }

//     pub fn min(self) -> F { self.x.min(self.y.min(self.z)) }
//     pub fn max(self) -> F { self.x.max(self.y.max(self.z)) }

//     pub fn argmin(self) -> UI {
//         if self.x < self.y && self.x < self.z { return X_AXIS }
//         if self.y < self.x && self.y < self.z { return Y_AXIS }
//         if self.z < self.x && self.z < self.y { return Z_AXIS }
//         assert_eq!(self.x, self.y);
//         assert_eq!(self.x, self.z);
//         return 0
//     }

//     pub fn argmax(self) -> UI {
//         if self.x > self.y && self.x > self.z { return X_AXIS }
//         if self.y > self.x && self.y > self.z { return Y_AXIS }
//         if self.z > self.x && self.z > self.y { return Z_AXIS }
//         assert_eq!(self.x, self.y);
//         assert_eq!(self.x, self.z);
//         return 0
//     }

//     pub fn permute(self, x: I, y: I, z: I) -> Vec3 {
//         Vec3 { x: self[x], y: self[y], z: self[z] }
//     }

//     pub fn as_coordinate_system(self) -> (Vec3, Vec3, Vec3) {
//         let v2;
//         if self.x.abs() > self.y.abs() {
//             v2 = Vec3{x: -self.z, y: 0.0, z: self.x} / (self.x * self.x + self.z * self.z).sqrt();
//         } else {
//             v2 = Vec3{x: 0.0, y: self.z, z: -self.y} / (self.y * self.y + self.z * self.z).sqrt();
//         }
//         let v3 = self.cross(v2);
//         (self, v2, v3)
//     }

//     pub fn distance(self, other: Vec3) -> F {
//         (self - other).length()
//     }

//     pub fn distance_squared(self, other: Vec3) -> F {
//         (self - other).length_squared()
//     }

//     pub fn lerp(self, other: Vec3, t: F) -> Vec3 {
//         (1.0 - t) * self + t * other
//     }

//     pub fn floor(self) -> Vec3 {
//         return Vec3 {
//             x: self.x.floor(),
//             y: self.y.floor(),
//             z: self.z.floor(),
//         }
//     }

//     pub fn ceil(self) -> Vec3 {
//         return Vec3 {
//             x: self.x.ceil(),
//             y: self.y.ceil(),
//             z: self.z.ceil(),
//         }
//     }

//     pub fn abs(self) -> Vec3 {
//         return Vec3 {
//             x: self.x.abs(),
//             y: self.y.abs(),
//             z: self.z.abs(),
//         }
//     }
// }

// impl Add<Vec3> for Vec3 {
//     type Output = Vec3;
//     fn add(self, other: Vec3) -> Self {
//         Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
//     }
// }

// impl Sub<Vec3> for Vec3 {
//     type Output = Vec3;
//     fn sub(self, other: Vec3) -> Self {
//         Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
//     }
// }

// impl Div<F> for Vec3 {
//     type Output = Self;
//     fn div(self, n: F) -> Self {
//         assert_ne!(n, 0.0);
//         Self {x: self.x / n, y: self.y / n, z: self.z / n}
//     }
// }

// impl Mul<F> for Vec3 {
//     type Output = Self;
//     fn mul(self, n: F) -> Self {
//         Self {x: self.x * n, y: self.y * n, z: self.z * n}
//     }
// }

// impl Mul<Vec3> for F {
//     type Output = Vec3;
//     fn mul(self, rhs: Vec3) -> Self::Output {
//         rhs * self
//     }
// }

// impl Neg for Vec3 {
//     type Output = Self;
//     fn neg(self) -> Self::Output {
//         Self {x: -self.x, y: -self.y, z: -self.z}
//     }
// }

// impl Index<I> for Vec3 {
//     type Output = F;
//     fn index(&self, index: I) -> &Self::Output {
//         match index {
//             0 => &self.x,
//             1 => &self.y,
//             2 => &self.z,
//             _ => panic!("Invalid index for Vec3/Point3/Normal3"),
//         }
//     }
// }

// impl IndexMut<I> for Vec3 {
//     fn index_mut(&mut self, index: I) -> &mut Self::Output {
//         match index {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             2 => &mut self.z,
//             _ => panic!("Invalid index for Vec3/Point3/Normal3"),
//         }
//     }
// }

// impl Add<Vec2> for Vec2 {
//     type Output = Vec2;
//     fn add(self, other: Vec2) -> Self {
//         Self {u: self.u + other.u, v: self.v + other.v}
//     }
// }

// impl Sub<Vec2> for Vec2 {
//     type Output = Vec2;
//     fn sub(self, other: Vec2) -> Self {
//         Self {u: self.u - other.u, v: self.v - other.v}
//     }
// }

// impl Div<F> for Vec2 {
//     type Output = Self;
//     fn div(self, n: F) -> Self {
//         assert_ne!(n, 0.0);
//         Self {u: self.u / n, v: self.v / n}
//     }
// }

// impl Mul<F> for Vec2 {
//     type Output = Self;
//     fn mul(self, n: F) -> Self {
//         Self {u: self.u * n, v: self.v * n}
//     }
// }

// impl Mul<Vec2> for F {
//     type Output = Vec2;
//     fn mul(self, rhs: Vec2) -> Self::Output {
//         rhs * self
//     }
// }

// impl Neg for Vec2 {
//     type Output = Self;
//     fn neg(self) -> Self::Output {
//         Self {u: -self.u, v: -self.v}
//     }
// }

// impl Index<I> for Vec2 {
//     type Output = F;
//     fn index(&self, index: I) -> &Self::Output {
//         match index {
//             0 => &self.u,
//             1 => &self.v,
//             _ => panic!("Invalid index for Vec2/Point2"),
//         }
//     }
// }

// impl IndexMut<I> for Vec2 {
//     fn index_mut(&mut self, index: I) -> &mut Self::Output {
//         match index {
//             0 => &mut self.u,
//             1 => &mut self.v,
//             _ => panic!("Invalid index for Vec2/Point2"),
//         }
//     }
// }
