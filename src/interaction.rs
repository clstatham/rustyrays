extern crate nalgebra_glm as glm;

use std::rc::Rc;

use crate::color::Color3;
use crate::common::*;
use crate::material::{Bsdf, Bxdf};
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::rng::RngGen;
use crate::vector::*;

#[derive(Debug, Default, Clone)]
pub struct Shading {
    pub n: Normal3,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    // pub dndu: Normal3,
    // pub dndv: Normal3,
}

// #[derive(Debug)]
#[derive(Clone)]
pub struct SurfaceInteraction {
    pub p: Point3,
    pub time: F,
    // pub p_error: Vec3,
    pub wo: Option<Vec3>,
    pub n: Option<Normal3>,

    pub uv: Option<Point2>,
    pub dpdu: Option<Vec3>,
    pub dpdv: Option<Vec3>,
    // pub dndu: Normal3,
    // pub dndv: Normal3,
    pub shading: Option<Shading>,

    pub primitive: Option<Rc<Primitive>>,
    pub bsdf: Option<Bsdf>,
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

    pub fn new(
        p: Point3,
        wo: Vec3,
        uv: Point2,
        dpdu: Vec3,
        dpdv: Vec3,
        time: F,
        primitive: Option<Rc<Primitive>>,
        bsdf: Option<Bsdf>,
    ) -> Self {
        let n = dpdu.cross(&dpdv).normalize();
        let mut out = Self {
            p,
            uv: Some(uv),
            wo: Some(wo),
            n: Some(n),
            dpdu: Some(dpdu),
            dpdv: Some(dpdv),
            // dndu,
            // dndv,
            time,
            shading: Some(Shading { n, dpdu, dpdv }),
            primitive,
            bsdf,
            // shape,
        };
        out.create_bsdf();
        
        // out.set_shading_geometry(dpdu, dpdv, false); // TODO: get actual authoritative value
        out
    }

    pub fn create_bsdf(&mut self) {
        self.bsdf = Some(Bsdf::new(self));
    }

    pub fn new_with_normal(
        p: Point3,
        wo: Vec3,
        uv: Point2,
        n: Normal3,
        time: F,
        primitive: Option<Rc<Primitive>>,
    ) -> Self {
        let mut out = Self {
            p,
            uv: Some(uv),
            wo: Some(wo),
            n: Some(n),
            dpdu: None,
            dpdv: None,
            time,
            // shape,
            primitive,
            shading: Some(Shading {
                n,
                dpdu: vec3(0.0, 0.0, 0.0),
                dpdv: vec3(0.0, 0.0, 0.0),
            }),
            bsdf: None,
        };
        out.create_bsdf();
        out
    }

    // pub fn le(&self, dir: &Vec3) -> Color3 {
    //     // if dir.dot(self.n.unwrap()) > 0.0 { self.primitive.unwrap().material.}
    // }

    pub fn new_general(p: Point3, time: F) -> Self {
        Self {
            p,
            time,
            wo: None,
            uv: None,
            n: None,
            dpdu: None,
            dpdv: None,
            shading: None,
            primitive: None,
            bsdf: None,
        }
    }

    pub fn add_bxdf(&mut self, bxdf: Rc<dyn Bxdf>) {
        if let Some(ref mut bsdf) = self.bsdf {
            bsdf.add(bxdf);
        }
    }

    pub fn scatter(&mut self, ray: &mut Ray, rng: &RngGen) {
        ray.origin = self.p;
        // ray.direction = self.bsdf.unwrap().sample_f(&self.p, &rng.uniform_sample_point2(), BXDF_ALL).unwrap();
        if let Some(primitive) = self.primitive.clone() {
            primitive.scatter(self, rng);
        }
    }

    // fn set_shading_geometry(&mut self, dpdus: Vec3, dpdvs: Vec3, authoritative: bool) {
    //     self.shading.n = dpdus.cross(&dpdvs).normalize();
    //     if authoritative {
    //         self.n = glm::faceforward(&self.n, &self.shading.n, &self.n);
    //     } else {
    //         self.shading.n = glm::faceforward(&self.shading.n, &self.n, &self.shading.n);
    //     }
    //     self.shading.dpdu = dpdus;
    //     self.shading.dpdv = dpdvs;
    //     // self.shading.dndu = dndus;
    //     // self.shading.dndv = dndvs;
    // }

    pub fn spawn_ray_to_point(&self, p: &Point3) -> Ray {
        Ray::new_non_differential(self.p, p - self.p, 0.0001, 0.9999, 0.0)
    }

    pub fn spawn_ray_to(&self, other: &SurfaceInteraction) -> Ray {
        // let w = other.p-self.p;
        self.spawn_ray_to_point(&other.p)
    }

    pub fn spawn_ray(&self, direction: Vec3) -> Ray {
        Ray::new_non_differential(self.p, direction, 0.0001, F::INFINITY, self.time)
    }
}
