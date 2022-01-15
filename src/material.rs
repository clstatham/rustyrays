use std::f32::consts::PI;
use std::rc::Rc;

use crate::color::{black, Color3};
use crate::common::*;
use crate::interaction::SurfaceInteraction;
use crate::ray::Ray;
use crate::sampler::Distribution1D;
use crate::texture::{ColorTexture, ScalarTexture};
use crate::vector::*;

pub const BXDF_REFLECTION: u8 = 0b00000001;
pub const BXDF_TRANSMISSION: u8 = 0b00000010;
pub const BXDF_DIFFUSE: u8 = 0b00000100;
pub const BXDF_GLOSSY: u8 = 0b00001000;
pub const BXDF_SPECULAR: u8 = 0b00010000;
pub const BXDF_ALL: u8 =
    BXDF_DIFFUSE | BXDF_GLOSSY | BXDF_REFLECTION | BXDF_SPECULAR | BXDF_TRANSMISSION;
pub type BXDFType = u8;

fn same_hemisphere(w: &Vec3, wp: &Vec3) -> bool {
    w.z * wp.z > 0.0
}
fn abs_cos_theta(w: &Vec3) -> F {
    w.z.abs()
}
pub trait Bxdf {
    fn bxdf_type(&self) -> BXDFType;
    fn scale(&self) -> F {
        1.0
    }
    fn f(&self, wo: &Vec3, wi: &Vec3) -> Option<Color3>;
    fn pdf(&self, wo: &Vec3, wi: &Vec3) -> F {
        if same_hemisphere(wo, wi) {
            abs_cos_theta(wi) / PI
        } else {
            0.0
        }
    }
    fn sample_f(&self, wo: &Vec3, u: &Point2) -> Option<(Color3, F, Vec3, BXDFType)> {
        let mut wi = Distribution1D::cosine_sample_hemisphere(u);
        // let mut wi_mut = *wi;
        if wo.z < 0.0 {
            wi.z *= -1.0
        }
        let pdf = self.pdf(wo, &wi);
        self.f(wo, &wi).map(|col| (col, pdf, wi, self.bxdf_type()))
    }
    fn rho(&self, n_samples: S, wo: &Vec3, samples: &[Point2]) -> Option<Color3>;
    fn rho_2samples(
        &self,
        n_samples: S,
        samples1: &[Point2],
        samples2: &[Point2],
    ) -> Option<Color3>;
}

pub struct LambertianReflection {
    r: Color3,
}

impl LambertianReflection {
    pub fn new(r: Color3) -> Self {
        Self { r }
    }
}

impl Bxdf for LambertianReflection {
    fn bxdf_type(&self) -> BXDFType {
        BXDF_DIFFUSE | BXDF_REFLECTION
    }
    fn rho_2samples(
        &self,
        n_samples: S,
        samples1: &[Point2],
        samples2: &[Point2],
    ) -> Option<Color3> {
        Some(self.r)
    }
    fn rho(&self, n_samples: S, wo: &Vec3, samples: &[Point2]) -> Option<Color3> {
        Some(self.r)
    }
    fn f(&self, wo: &Vec3, wi: &Vec3) -> Option<Color3> {
        Some(self.r / PI)
    }
}

// impl BitAnd for dyn BXDF {
//     type Output = BXDFType;

//     fn bitand(self, rhs: Self) -> Self::Output {
//         self.bxdf_type() & rhs.bxdf_type()
//     }
// }

#[derive(Clone)]
pub struct Bsdf {
    // shape: Rc<dyn Shape>,
    // material: Rc<'a dyn& (BSDFMaterial + 'a)>,
    // pub materials: Vec<Rc<dyn Material>>,
    // pub material: Rc<dyn Material>,
    pub bxdfs: Vec<Rc<dyn Bxdf>>,
    // pub wo: Vec3,
    // pub wi: Vec3,
    // pub attenuation: Color3,
    // pub pdf_value: F,
    ss: Vec3,
    ns: Vec3,
    ng: Vec3,
    ts: Vec3,
}

impl Bsdf {
    pub fn new(inter: &SurfaceInteraction) -> Self {
        Self { // TODO: remove all these unwraps
            bxdfs: vec![],
            ns: inter.shading.as_ref().unwrap().n,
            ng: inter.n.unwrap(),
            ss: inter.shading.as_ref().unwrap().dpdu.normalize(),
            ts: inter.shading.as_ref().unwrap().n.cross(&inter.shading.as_ref().unwrap().dpdu.normalize())
        }
    }

    pub fn add(&mut self, bxdf: Rc<dyn Bxdf>) {
        self.bxdfs.push(bxdf);
    }

    pub fn pdf(&self, wo: &Vec3, wi: &Vec3, flags: BXDFType) -> F {
        self.bxdfs.iter().filter_map(|item| 
            match item {
                bxdf if bxdf.bxdf_type() & flags != 0 => Some(bxdf.pdf(wo, wi)),
                _ => None
            }
        ).product()
    }

    pub fn f(&self, wo_world: &Vec3, wi_world: &Vec3, flags: BXDFType) -> Color3 {
        let wo = self.world_to_local(wo_world);
        let wi = self.world_to_local(wi_world);
        let reflect = wi_world.dot(&self.ng) * wo_world.dot(&self.ng) > 0.0;
        let mut f = black();
        for bxdf in self.bxdfs.iter() {
            if bxdf.bxdf_type() & flags != 0 && (
                (reflect && (bxdf.bxdf_type() & BXDF_REFLECTION != 0)) ||
                (!reflect && (bxdf.bxdf_type() & BXDF_TRANSMISSION != 0))
            ) {
                if let Some(f_col) = bxdf.f(&wo, &wi) {
                    f += f_col;
                }
            }
        }
        f
    }
    pub fn sample_f(&self, wo: &Vec3, u: &Point2, flags: BXDFType) -> Option<(Color3, F, Vec3, BXDFType)> {
        let mut matching_comps = 0;
        if self.bxdfs.is_empty() { return None }
        for bxdf in self.bxdfs.iter() {
            if bxdf.bxdf_type() & flags != 0 {
                matching_comps += 1;
            }
        }
        let comp;
        if matching_comps == 0 { comp = 0; }
        else {
            comp = ((u.x * matching_comps as F).floor() as I).min(matching_comps - 1);
        }
        let bxdf = &self.bxdfs[comp as S];
        // let u_remapped = point2(u[0] * matching_comps as F - comp as F, u[1]);
        // let
        bxdf.sample_f(wo, u)
    }

    pub fn world_to_local(&self, v: &Vec3) -> Vec3 { vec3(self.ss.dot(v), self.ts.dot(v), self.ns.dot(v)) }
    pub fn local_to_world(&self, v: &Vec3) -> Vec3 { vec3(
        self.ss.x * v.x + self.ts.x * v.y + self.ns.x * v.z,
        self.ss.y * v.x + self.ts.y * v.y + self.ns.y * v.z,
        self.ss.z * v.x + self.ts.z * v.y + self.ns.z * v.z,
    ) }
}

pub trait Material {
    fn calculate_bsdf(
        &self,
        inter: &mut SurfaceInteraction,
        // rng: &RngGen,
    );
    fn scattering_pdf(&self, _ray: &Ray, _inter: &SurfaceInteraction) -> F;
}

#[derive(Clone)]
pub struct Matte {
    pub kd: Rc<dyn ColorTexture>,
    pub sigma: Option<Rc<dyn ScalarTexture>>,
    pub bump_map: Option<Rc<dyn ScalarTexture>>,
}

impl Material for Matte {
    fn calculate_bsdf(
        &self,
        inter: &mut SurfaceInteraction,
        // rng: &RngGen,
    ) {
        // let d = Distribution1D::cosine_sample_hemisphere(&point2(rng.sample_0_1(), rng.sample_0_1()));
        // let onb = ONB::new_from_w(&inter.n);
        // let new_d = onb.local(&d);
        // ray.direction = new_d;
        // let cosine = new_d.dot(&onb.w());

        let r = self.kd.eval(inter);
        // let adjusted_direction = vec3(d.x*inter.n.x, d.y*inter.n.y, d.z*inter.n.z);
        // let mut bxdfs: Vec<Rc<dyn Bxdf>> = vec![];
        if r != black() {
            match &self.sigma {
                Some(sigma) => {
                    let sig = sigma.eval(inter).clamp(0.0, 90.0);
                    if sig == 0.0 {
                        inter.add_bxdf(Rc::new(LambertianReflection::new(r)));
                    } else {
                        // interacted_materials.push(Rc::new(OrenNayar::new(r, sig)));
                    }
                }
                None => {}
            }
        }
        
    }

    fn scattering_pdf(&self, ray: &Ray, inter: &SurfaceInteraction) -> F {
        if let Some(n) = inter.n {
            match n.dot(&ray.direction) {
                cos_theta if cos_theta > 0.0 => Distribution1D::cosine_hemisphere_pdf(cos_theta),
                _ => 0.0,
            }
        } else {
            0.0
        }
    }
}
