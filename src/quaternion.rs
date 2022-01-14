use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use crate::common::*;
use crate::vector::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub v: Vec3,
    pub w: F,
}

impl Quaternion {
    pub fn new(v: Vec3, w: F) -> Self { Self {v, w} }

    pub fn x(self) -> F { self.v.x }
    pub fn y(self) -> F { self.v.y }
    pub fn z(self) -> F { self.v.z }
    pub fn w(self) -> F { self.w }

    pub fn dot(self, q: Quaternion) -> F {
        self.v.dot(&q.v) + self.w * q.w
    }

    pub fn normalize(self) -> Quaternion {
        self / self.dot(self).sqrt()
    }

    pub fn slerp(self, a: Quaternion, t: F) -> Quaternion {
        let cos_theta = self.dot(a);
        if cos_theta > 0.9995 {
            ((1.0 - t) * self + t * a).normalize()
        } else {
            let theta = cos_theta.clamp(-1.0, 1.0).acos();
            let thetap = theta * t;
            let qperp = (a - self * cos_theta).normalize();
            self * thetap.cos() + qperp * thetap.sin()
        }
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            v: self.v + rhs.v,
            w: self.w + rhs.w,
        }
    }
}

impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            v: self.v - rhs.v,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<F> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: F) -> Self::Output {
        Quaternion {
            v: self.v * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Quaternion> for F {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            v: rhs.v * self,
            w: rhs.w * self,
        }
    }
}

impl Div<F> for Quaternion {
    type Output = Quaternion;
    fn div(self, rhs: F) -> Self::Output {
        assert_ne!(rhs, 0.0);
        Quaternion {
            v: self.v / rhs,
            w: self.w / rhs,
        }
    }
}
