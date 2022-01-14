use std::f32::consts::PI;
use std::ops::BitAnd;
use std::rc::Rc;

use crate::color::{Color3, black};
use crate::common::*;
use crate::interaction::SurfaceInteraction;
use crate::ray::Ray;
use crate::rng::RngGen;
use crate::sampler::Distribution1D;
use crate::shape::Shape;
use crate::texture::{ColorTexture, ScalarTexture};
use crate::vector::*;
use crate::onb::ONB;


pub const BXDF_REFLECTION: u8 =     0b00000001;
pub const BXDF_TRANSMISSION: u8 =   0b00000010;
pub const BXDF_DIFFUSE: u8 =        0b00000100;
pub const BXDF_GLOSSY: u8 =         0b00001000;
pub const BXDF_SPECULAR: u8 =       0b00010000;
pub const BXDF_ALL: u8 = BXDF_DIFFUSE | BXDF_GLOSSY | BXDF_REFLECTION | BXDF_SPECULAR | BXDF_TRANSMISSION;
pub type BXDFType = u8;

fn same_hemisphere(w: &Vec3, wp: &Vec3) -> bool { w.z * wp.z > 0.0 }
fn abs_cos_theta(w: &Vec3) -> F { w.z.abs() }
pub trait BXDF {
    fn bxdf_type(&self) -> BXDFType;
    fn scale(&self) -> F { 1.0 }
    fn f(&self, wo: &Vec3, wi: &Vec3) -> Option<Color3>;
    fn pdf(&self, wo: &Vec3, wi: &Vec3) -> F {
        if same_hemisphere(wo, wi) { abs_cos_theta(wi) / PI} else { 0.0 }
    }
    fn sample_f(&self, wo: &Vec3, u: &Point2) -> Option<(Color3, F, Vec3, BXDFType)> {
        let mut wi = Distribution1D::cosine_sample_hemisphere(u);
        if wo.z < 0.0 { wi.z *= -1.0 }
        let pdf = self.pdf(wo, &wi);
        match self.f(wo, &wi) {
            Some(col) => Some((col, pdf, wi, self.bxdf_type())),
            None => None
        }
    }
    fn rho(&self, n_samples: S, wo: &Vec3, samples: &[Point2]) -> Option<Color3>;
    fn rho_2samples(&self, n_samples: S, samples1: &[Point2], samples2: &[Point2]) -> Option<Color3>;
}


pub struct LambertianReflection {
    r: Color3,
}

impl LambertianReflection {
    pub fn new(r: Color3) -> Self {
        Self {r}
    }
}

impl BXDF for LambertianReflection {
    fn bxdf_type(&self) -> BXDFType {
        BXDF_DIFFUSE | BXDF_REFLECTION
    }
    fn rho_2samples(&self, n_samples: S, samples1: &[Point2], samples2: &[Point2]) -> Option<Color3> {
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

pub struct ScatterResult {
    // shape: Rc<dyn Shape>,
    // material: Rc<'a dyn& (BSDFMaterial + 'a)>,
    // pub materials: Vec<Rc<dyn Material>>,
    pub material: Rc<dyn Material>,
    pub bxdfs: Vec<Rc<dyn BXDF>>,
    pub wo: Vec3,
    // pub attenuation: Color3,
    // pub pdf_value: F,
}

pub trait Material {
    fn scatter_ray(&self, ray: &mut Ray, inter: &SurfaceInteraction, rng: &RngGen) -> Option<ScatterResult>;
    fn scattering_pdf(&self, _ray: &Ray, _inter: &SurfaceInteraction) -> F;
}


#[derive(Clone)]
pub struct Matte {
    pub kd: Rc<dyn ColorTexture>,
    pub sigma: Option<Rc<dyn ScalarTexture>>,
    pub bump_map: Option<Rc<dyn ScalarTexture>>,
}

impl Material for Matte {
    fn scatter_ray(&self, ray: &mut Ray, inter: &SurfaceInteraction, rng: &RngGen) -> Option<ScatterResult> {
        
        // let d = Distribution1D::cosine_sample_hemisphere(&point2(rng.sample_0_1(), rng.sample_0_1()));
        // let onb = ONB::new_from_w(&inter.n);
        // let new_d = onb.local(&d);
        // ray.direction = new_d;
        // let cosine = new_d.dot(&onb.w());
        
        let r = self.kd.eval(inter);
        // let adjusted_direction = vec3(d.x*inter.n.x, d.y*inter.n.y, d.z*inter.n.z);
        let mut bxdfs: Vec<Rc<dyn BXDF>> = vec![];
        if r != black() {
            match &self.sigma {
                Some(sigma) => {
                    let sig = sigma.eval(inter).clamp(0.0, 90.0);
                    if sig == 0.0 {
                        bxdfs.push(Rc::new(LambertianReflection::new(r)));
                    } else {
                        // interacted_materials.push(Rc::new(OrenNayar::new(r, sig)));
                    }
                },
                None => {return None}
            }            
        }
        
        Some(ScatterResult {
            // shape: inter.shape.clone(),
            material: Rc::new(self.clone()),
            bxdfs,
            wo: -ray.direction,
            // attenuation: r,
        })
    }

    fn scattering_pdf(&self, ray: &Ray, inter: &SurfaceInteraction) -> F {
        let cos_theta = inter.n.dot(&ray.direction);
        Distribution1D::cosine_hemisphere_pdf(cos_theta)
    }
}

