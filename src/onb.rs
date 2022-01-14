use crate::vector::*;

pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: &Vec3) -> Vec3 {
        let ux = self.u() * a.x;
        let vy = self.v() * a.y;
        let wz = self.w() * a.z;
        ux + vy + wz
    }

    pub fn new_from_w(n: &Vec3) -> Self {
        let w = n.normalize();
        let a = match w.x.abs() > 0.9 {
            true => vec3(0.0, 1.0, 0.0),
            false => vec3(1.0, 0.0, 0.0),
        };
        let v = w.cross(&a).normalize();
        let u = w.cross(&v);
        Self { axis: [u, v, w] }
    }
}
