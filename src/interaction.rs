extern crate nalgebra_glm as glm;

use std::rc::Rc;

use crate::common::*;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vector::*;

#[derive(Debug, Default)]
pub struct Shading {
    pub n: Normal3,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    // pub dndu: Normal3,
    // pub dndv: Normal3,
}

// #[derive(Debug)]
pub struct SurfaceInteraction {
    pub p: Point3,
    pub time: F,
    // pub p_error: Vec3,
    // pub wo: Vec3,
    pub n: Normal3,

    pub uv: Point2,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    // pub dndu: Normal3,
    // pub dndv: Normal3,

    pub shading: Shading,

    // pub shape: Rc<dyn Shape>
}

// impl Default for SurfaceInteraction {
//     fn default() -> Self {
//         Self {
//             p: Point3::default(),
//             time: 0.0,
//             p_error: Vec3::default(),
//             wo: Vec3::default(),
//             n: Normal3::default(),
//             uv: Point2::default(),
//             dpdu: Vec3::default(),
//             dpdv: Vec3::default(),
//             dndu: Normal3::default(),
//             dndv: Normal3::default(),
//             shading: Shading::default(),
//         }
//     }
// }

impl SurfaceInteraction {
    // pub fn is_surface_interaction(self) -> bool { self.n != Normal3::default() }

    pub fn new(p: Point3, uv: Point2, dpdu: Vec3, dpdv: Vec3, time: F) -> Self {
        let n = dpdu.cross(&dpdv).normalize();
        let mut out = Self {
            p,
            uv,
            // wo,
            n,
            dpdu,
            dpdv,
            // dndu,
            // dndv,
            time,
            shading: Shading { n, dpdu, dpdv, },
            // shape,
        };
        out.set_shading_geometry(dpdu, dpdv, false); // TODO: get actual authoritative value
        out
    }

    pub fn new_with_normal(p: Point3, uv: Point2, n: Normal3, time: F) -> Self {
        Self {
            p,
            uv,
            // wo,
            n,
            dpdu: vec3(0.0,0.0,0.0),
            dpdv: vec3(0.0,0.0,0.0),
            time,
            // shape,
            shading: Shading { n, dpdu: vec3(0.0,0.0,0.0), dpdv: vec3(0.0,0.0,0.0) }
        }
    }

    fn set_shading_geometry(&mut self, dpdus: Vec3, dpdvs: Vec3, authoritative: bool) {
        self.shading.n = dpdus.cross(&dpdvs).normalize();
        if authoritative {
            self.n = glm::faceforward(&self.n, &self.shading.n, &self.n);
        } else {
            self.shading.n = glm::faceforward(&self.shading.n, &self.n, &self.shading.n);
        }
        self.shading.dpdu = dpdus;
        self.shading.dpdv = dpdvs;
        // self.shading.dndu = dndus;
        // self.shading.dndv = dndvs;
    }
}