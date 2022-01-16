use std::ops::Mul;

extern crate nalgebra as na;
extern crate nalgebra_glm as glm;

use glm::rotation;

use crate::aabb::AABB3;
use crate::common::*;
use crate::interaction::Shading;
use crate::interaction::Interaction;
use crate::matrix::*;
use crate::ray::Ray;
use crate::vector::*;

// pub trait Transform {
//     fn forward_matrix(self) -> Matrix4;
//     fn inverse_matrix(self) -> Matrix4;
// }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    m_forward: Matrix4,
    m_inverse: Matrix4,
}

impl Transform {
    pub fn forward_matrix(&self) -> &Matrix4 {
        &self.m_forward
    }

    pub fn inverse_matrix(&self) -> &Matrix4 {
        &self.m_inverse
    }

    pub fn as_inverse_transform(&self) -> Transform {
        Transform {
            m_forward: self.m_inverse,
            m_inverse: self.m_forward,
        }
    }

    pub fn new(m_forward: Matrix4, m_inverse: Matrix4) -> Self {
        Self {
            m_forward,
            m_inverse,
        }
    }
    pub fn new_from_forward(m_forward: Matrix4) -> Option<Self> {
        let m_inverse = m_forward.try_inverse();
        m_inverse.map(|m_inv| Self {
            m_forward,
            m_inverse: m_inv,
        })
    }

    pub fn from_transpose(t: Transform) -> Self {
        Self {
            m_forward: t.forward_matrix().transpose(),
            m_inverse: t.inverse_matrix().transpose(),
        }
    }

    pub fn new_identity() -> Self {
        Self {
            m_forward: Matrix4::identity(),
            m_inverse: Matrix4::identity(),
        }
    }

    pub fn new_translate(delta: Vec3) -> Self {
        Self {
            // m_forward: matrix4([
            //     [1.0, 0.0, 0.0, delta.x],
            //     [0.0, 1.0, 0.0, delta.y],
            //     [0.0, 0.0, 1.0, delta.z],
            //     [0.0, 0.0, 0.0, 1.0]
            // ]),
            m_forward: glm::translation(&delta),
            // m_inverse: matrix4([
            //     [1.0, 0.0, 0.0, -delta.x],
            //     [0.0, 1.0, 0.0, -delta.y],
            //     [0.0, 0.0, 1.0, -delta.z],
            //     [0.0, 0.0, 0.0, 1.0]
            // ]),
            m_inverse: glm::translation(&delta)
                .try_inverse()
                .expect("Couldn't invert translation matrix!"),
        }
    }

    pub fn new_scale(s: Vec3) -> Self {
        Self {
            // m_forward: Matrix4::new([
            //     [s.x, 0.0, 0.0, 0.0],
            //     [0.0, s.y, 0.0, 0.0],
            //     [0.0, 0.0, s.z, 0.0],
            //     [0.0, 0.0, 0.0, 1.0],
            // ]),
            // m_inverse: Matrix4::new([
            //     [1.0/s.x, 0.0, 0.0, 0.0],
            //     [0.0, 1.0/s.y, 0.0, 0.0],
            //     [0.0, 0.0, 1.0/s.z, 0.0],
            //     [0.0, 0.0, 0.0, 1.0],
            // ])
            m_forward: glm::scaling(&s),
            m_inverse: glm::scaling(&s)
                .try_inverse()
                .expect("Couldn't build inverse scaling matrix"),
        }
    }

    pub fn has_scale(&self) -> bool {
        let la2 = &self.fvec(&vec3(1.0, 0.0, 0.0)).magnitude_squared();
        let lb2 = &self.fvec(&vec3(0.0, 1.0, 0.0)).magnitude_squared();
        let lc2 = &self.fvec(&vec3(0.0, 0.0, 1.0)).magnitude_squared();
        let not_one = |x: &F| *x < 0.999 || *x > 1.001;
        not_one(la2) || not_one(lb2) || not_one(lc2)
    }

    pub fn new_rotate_x(theta: F) -> Self {
        // let sin_theta = theta.sin();
        // let cos_theta = theta.cos();
        // let m_forward = Matrix4::new([
        //     [1.0, 0.0, 0.0, 0.0],
        //     [0.0, cos_theta, -sin_theta, 0.0],
        //     [0.0, sin_theta, cos_theta, 0.0],
        //     [0.0, 0.0, 0.0, 1.0],
        // ]);
        // Self { m_forward, m_inverse: m_forward.invert().expect("Couldn't invert rotate_x matrix!") }
        Self {
            m_forward: rotation(theta, &Vec3::x_axis()),
            m_inverse: rotation(-theta, &Vec3::x_axis()),
        }
    }

    pub fn new_rotate_y(theta: F) -> Self {
        // let sin_theta = theta.sin();
        // let cos_theta = theta.cos();
        // let m_forward = Matrix4::new([
        //     [cos_theta, 0.0, sin_theta, 0.0],
        //     [0.0, 1.0, 0.0, 0.0],
        //     [-sin_theta, 0.0, cos_theta, 0.0],
        //     [0.0, 0.0, 0.0, 1.0],
        // ]);
        // Self { m_forward, m_inverse: m_forward.invert().expect("Couldn't invert rotate_y matrix!") }
        Self {
            m_forward: rotation(theta, &Vec3::y_axis()),
            m_inverse: rotation(-theta, &Vec3::y_axis()),
        }
    }

    pub fn new_rotate_z(theta: F) -> Self {
        // let sin_theta = theta.sin();
        // let cos_theta = theta.cos();
        // let m_forward = Matrix4::new([
        //     [cos_theta, -sin_theta, 0.0, 0.0],
        //     [sin_theta, cos_theta, 0.0, 0.0],
        //     [0.0, 0.0, 1.0, 0.0],
        //     [0.0, 0.0, 0.0, 1.0],
        // ]);
        // Self { m_forward, m_inverse: m_forward.invert().expect("Couldn't invert rotate_z matrix!") }
        Self {
            m_forward: rotation(theta, &Vec3::z_axis()),
            m_inverse: rotation(-theta, &Vec3::z_axis()),
        }
    }

    // TODO
    // pub fn new_rotate(theta: F, axis: Vec3) -> Self {
    //     let a = axis.normalize();
    //     let sin_theta = theta.sin();
    //     let cos_theta = theta.cos();
    // }

    pub fn new_lookat(pos: Point3, look: Point3, up: Vec3) -> Self {
        // let dir = (look - pos).normalize();
        // let right = up.normalize().cross(&dir).normalize();
        // let new_up = dir.cross(&right);
        // let m_forward = matrix4([
        //     [right.x, new_up.x, dir.x, pos.x],
        //     [right.y, new_up.y, dir.y, pos.y],
        //     [right.z, new_up.z, dir.z, pos.z],
        //     [0.0, 0.0, 0.0, 1.0],
        // ]);
        // Self { m_forward, m_inverse: m_forward.invert().expect("Couldn't invert lookat matrix!") }
        let m_forward = glm::look_at(
            &vec3(pos.x, pos.y, pos.z),
            &vec3(look.x, look.y, look.z),
            &up,
        );
        Self {
            m_forward,
            m_inverse: m_forward
                .try_inverse()
                .expect("Couldn't invert lookat matrix!"),
        }
    }

    pub fn new_orthographic(z_near: F, z_far: F) -> Self {
        Transform::new_scale(vec3(1.0, 1.0, 1.0 / (z_far - z_near)))
            * Transform::new_translate(vec3(0.0, 0.0, -z_near))
        // Self {
        //     m_forward: glm::ortho_rh()
        // }
    }

    pub fn new_perspective(fov_deg: F, n: F, f: F) -> Self {
        let persp = matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, f / (f - n), -f * n / (f - n)],
            [0.0, 0.0, 1.0, 0.0],
        ]);
        let inv_tan = 1.0 / (deg2rad(fov_deg) / 2.0).tan();
        Transform::new_scale(vec3(inv_tan, inv_tan, 1.0))
            * Transform::new_from_forward(persp).expect("Couldn't invert perspective matrix!")
        // Self {
        //     m_forward: glm::perspective_fov(deg2rad(fov_deg / 2.0), 1.0, 1.0, n, f),
        //     m_inverse: glm::perspective_fov(deg2rad(fov_deg / 2.0), 1.0, 1.0, n, f).try_inverse().expect("Couldn't invert perspective matrix!"),
        // }
    }

    pub fn transpose(self) -> Transform {
        Transform::new(self.m_forward.transpose(), self.m_inverse.transpose())
    }

    pub fn swaps_handedness(self) -> bool {
        let m = self.m_forward;
        // let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) -
        //               m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) +
        //               m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
        todo!();
    }

    /// IMPORTANT: Only use for Points!
    pub fn forward_point_transform(self, a: Point3) -> Point3 {
        from_na_point3(self.m_forward.transform_point(&to_na_point3(a)))
    }
    /// IMPORTANT: Only use for Points!
    pub fn fpt(self, a: Point3) -> Point3 {
        self.forward_point_transform(a)
    }
    /// IMPORTANT: Only use for Points!
    pub fn inverse_point_transform(self, a: Point3) -> Point3 {
        from_na_point3(self.m_inverse.transform_point(&to_na_point3(a)))
    }
    /// IMPORTANT: Only use for Points!
    pub fn ipt(self, a: Point3) -> Point3 {
        self.inverse_point_transform(a)
    }

    /// IMPORTANT: Only use for Vectors!
    pub fn forward_vector_transform(self, a: &Vec3) -> Vec3 {
        self.m_forward.transform_vector(a)
        // else { point3(x, y, z) / w }
    }
    /// IMPORTANT: Only use for Vectors!
    pub fn fvec(self, a: &Vec3) -> Vec3 {
        self.forward_vector_transform(a)
    }
    /// IMPORTANT: Only use for Vectors!
    pub fn inverse_vector_transform(self, a: &Vec3) -> Vec3 {
        self.m_inverse.transform_vector(a)
    }
    /// IMPORTANT: Only use for Vectors!
    pub fn ivec(self, a: &Vec3) -> Vec3 {
        self.inverse_vector_transform(a)
    }

    /// IMPORTANT: Only use for Normals!
    // pub fn forward_normal_transform(self, a: Normal3) -> Normal3 {
    //     let m = self.m_inverse;
    //     let x = m[0][0]*a.x + m[1][0]*a.y + m[2][0]*a.z;
    //     let y = m[0][1]*a.x + m[1][1]*a.y + m[2][1]*a.z;
    //     let z = m[0][2]*a.x + m[1][2]*a.y + m[2][2]*a.z;
    //     // let w = m[3][0]*a.x + m[3][1]*a.y + m[3][2]*a.z + m[3][3];
    //     normal3(x, y, z)
    //     // else { point3(x, y, z) / w }
    // }
    // /// IMPORTANT: Only use for Normals!
    pub fn fnorm(self, a: &Normal3) -> Normal3 {
        self.forward_vector_transform(a)
    }
    // /// IMPORTANT: Only use for Normals!
    // pub fn inverse_normal_transform(self, a: Normal3) -> Normal3 {
    //     let m = self.m_forward;
    //     let x = m[0][0]*a.x + m[0][1]*a.y + m[0][2]*a.z;
    //     let y = m[1][0]*a.x + m[1][1]*a.y + m[1][2]*a.z;
    //     let z = m[2][0]*a.x + m[2][1]*a.y + m[2][2]*a.z;
    //     // let w = m[3][0]*a.x + m[3][1]*a.y + m[3][2]*a.z + m[3][3];
    //     normal3(x, y, z)
    // }
    // /// IMPORTANT: Only use for Normals!
    pub fn inorm(self, a: &Normal3) -> Normal3 {
        self.inverse_vector_transform(a)
    }

    pub fn forward_ray_transform(self, a: &Ray) -> Ray {
        let o = self.fpt(a.origin);
        let d = self.fvec(&a.direction);
        Ray {
            origin: o,
            direction: d,
            ..*a
        }
    }
    pub fn fray(self, a: &Ray) -> Ray {
        self.forward_ray_transform(a)
    }

    pub fn inverse_ray_transform(self, a: &Ray) -> Ray {
        let o = self.ipt(a.origin);
        let d = self.ivec(&a.direction);
        Ray {
            origin: o,
            direction: d,
            ..*a
        }
    }
    pub fn iray(self, a: &Ray) -> Ray {
        self.inverse_ray_transform(a)
    }

    // TODO: Optimize these?
    pub fn forward_aabb_transform(self, a: &AABB3) -> AABB3 {
        let mut ret = AABB3::from(point3(a.p_min.x, a.p_min.y, a.p_min.z));

        ret = ret.union(self.fpt(point3(a.p_max.x, a.p_min.y, a.p_min.z)));
        ret = ret.union(self.fpt(point3(a.p_min.x, a.p_max.y, a.p_min.z)));
        ret = ret.union(self.fpt(point3(a.p_min.x, a.p_min.y, a.p_max.z)));

        ret = ret.union(self.fpt(point3(a.p_min.x, a.p_max.y, a.p_max.z)));
        ret = ret.union(self.fpt(point3(a.p_max.x, a.p_max.y, a.p_min.z)));
        ret = ret.union(self.fpt(point3(a.p_min.x, a.p_max.y, a.p_min.z)));

        ret = ret.union(self.fpt(point3(a.p_max.x, a.p_min.y, a.p_max.z)));
        ret = ret.union(self.fpt(point3(a.p_max.x, a.p_max.y, a.p_max.z)));
        ret
    }
    pub fn faabb(self, a: &AABB3) -> AABB3 {
        self.forward_aabb_transform(a)
    }

    pub fn inverse_aabb_transform(self, a: &AABB3) -> AABB3 {
        let mut ret = AABB3::from(point3(a.p_min.x, a.p_min.y, a.p_min.z));

        ret = ret.union(self.ipt(point3(a.p_max.x, a.p_min.y, a.p_min.z)));
        ret = ret.union(self.ipt(point3(a.p_min.x, a.p_max.y, a.p_min.z)));
        ret = ret.union(self.ipt(point3(a.p_min.x, a.p_min.y, a.p_max.z)));

        ret = ret.union(self.ipt(point3(a.p_min.x, a.p_max.y, a.p_max.z)));
        ret = ret.union(self.ipt(point3(a.p_max.x, a.p_max.y, a.p_min.z)));
        ret = ret.union(self.ipt(point3(a.p_min.x, a.p_max.y, a.p_min.z)));

        ret = ret.union(self.ipt(point3(a.p_max.x, a.p_min.y, a.p_max.z)));
        ret = ret.union(self.ipt(point3(a.p_max.x, a.p_max.y, a.p_max.z)));
        ret
    }
    pub fn iaabb(self, a: &AABB3) -> AABB3 {
        self.inverse_aabb_transform(a)
    }

    pub fn forward_surface_interaction_transform(
        self,
        a: Interaction,
    ) -> Interaction {
        let dpdu;
        let dpdv;
        let n;
        let shading;
        let wo;
        if let Some(a_dpdu) = a.dpdu {
            dpdu = Some(self.fvec(&a_dpdu));
        } else {
            dpdu = None;
        }
        if let Some(a_dpdv) = a.dpdv {
            dpdv = Some(self.fvec(&a_dpdv));
        } else {
            dpdv = None;
        }
        if let Some(a_n) = a.n {
            n = Some(self.fnorm(&a_n).normalize());
        } else {
            n = None;
        }
        if let Some(a_wo) = a.wo {
            wo = Some(self.fvec(&a_wo));
        } else {
            wo = None;
        }
        if let Some(a_shading) = a.shading {
            shading = Some(Shading {
                n: self.fnorm(&a_shading.n),
                dpdu: self.fvec(&a_shading.dpdu),
                dpdv: self.fvec(&a_shading.dpdv),
                // dndu: self.fnorm(a.shading.dndu),
                // dndv: self.fnorm(a.shading.dndv),
            });
        } else {
            shading = None;
        }
        Interaction {
            p: self.fpt(a.p),
            n,
            wo,
            time: a.time,
            uv: a.uv,
            dpdu,
            dpdv,
            // dndu: self.fnorm(a.dndu),
            // dndv: self.fnorm(a.dndv),
            shading,
            primitive: a.primitive,
            // p_error: a.p_error,
            // shape: a.shape,
            bsdf: a.bsdf,
            // medium_interface: a.medium_interface,
        }
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Transform {
        Transform::new(
            self.m_forward * rhs.m_forward,
            self.m_inverse * rhs.m_inverse,
        )
    }
}

// impl From<Quaternion> for Transform {
//     fn from(q: Quaternion) -> Self {
//         let x = q.x();
//         let y = q.y();
//         let z = q.z();
//         let w = q.w();
//         let m_forward = Matrix4::new([
//             [1.0-2.0*(y*y-z*z), 2.0*(x*y+z*w), 2.0*(x*z-y*w), 0.0],
//             [2.0*(x*y-z*w), 1.0-2.0*(x*x+z*z), 2.0*(y*z+x*w), 0.0],
//             [2.0*(x*z+y*w), 2.0*(y*z-x*w), 1.0-2.0*(x*x+y*y), 0.0],
//             [0.0, 0.0, 0.0, 1.0],
//         ]);
//         Self { m_forward, m_inverse: m_forward.invert().expect("Error inverting Transform::from<Quaternion>") }
//     }
// }

/* TODO: WIP
pub struct AnimatedTransform {
    start_transform: Box<Transform>,
    end_transform: Box<Transform>,
    start_time: F,
    end_time: F,
    actually_animated: bool,
    t: [Vec3; 2],
    r: [Quaternion; 2],
    s: [Matrix4; 2],
    has_rotation: bool,
}

fn decompose_transform(tr: Transform) -> Option<(Vec3, Quaternion, Matrix4)> {
    let m = tr.m_forward;
    let t = vec3(m[0][3], m[1][3], m[2][3]);
    let mut m1 = m;
    for i in 1..3 { m1[i][3] = 0.0; m1[3][i] = 0.0; }
    m1[3][3] = 1.0;
    let mut norm: F = 1.0;
    let mut count: I = 0;
    let mut r = m1;
    while count < 100 && norm > 0.0001 {
        let mut r_next = Matrix4::default();
        let rit_ = r.transpose().invert();
        match rit_ {
            None => return None,
            Some(rit) => {
                for i in 0..4 {
                    for j in 0..4 {
                        r_next[i][j] = 0.5 * (r[i][j] + rit[i][j]);
                    }
                }
                norm = 0.0;
                for i in 0..3 {
                    let n = (r[i][0] - r_next[i][0]).abs() +
                                (r[i][1] - r_next[i][1]).abs() +
                                (r[i][2] - r_next[i][2]).abs();
                    norm = norm.max(n);
                }
                count += 1;
                r = r_next;
            }

        }
    }
    let mut s_ = r.invert();
    match s_ {
        None => return None,
        Some(mut s) => {
            s = s * m;
            if
        }
    }
}

impl AnimatedTransform {
    pub fn new(start_transform: Transform, start_time: F, end_transform: Transform, end_time: F) -> Self {
        let (t0, r0, s0) = decompose_transform(start_transform);
        let (t1, r1, s1) = decompose_transform(end_transform);

        Self {
            start_transform: Box::new(start_transform),
            end_transform: Box::new(end_transform),
            start_time,
            end_time,
            actually_animated: start_transform != end_transform,
            t: [t0, t1],
            r: [r0, r1],
            s: [s0, s1],
            has_rotation: r0.dot(r1) < 0.9995,
        }
    }
}

*/
