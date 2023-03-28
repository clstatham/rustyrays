use std::{f64::consts::PI, sync::Arc};

use crate::{vector::*, common::F, ray::Ray, rng::RngGen, color::Color3};

pub trait PhaseFunction {
    fn p(&self, wo: &Vec3, wi: &Vec3) -> F;
    fn sample_p(&self, wo: &Vec3, u: &Point2) -> (Vec3, F);
}

pub fn phase_hg(cos_theta: F, g: F) -> F {
    let denom = 1.0 + g*g + 2.0*g*cos_theta;
    (1.0 / (4.0 * PI)) * (1.0 - g*g) / (denom * denom.sqrt())
}

pub struct HenyeyGreenstein {
    pub g: F,
}

impl PhaseFunction for HenyeyGreenstein {
    fn p(&self, wo: &Vec3, wi: &Vec3) -> F {
        phase_hg(wo.dot(wi), self.g)
    }

    fn sample_p(&self, wo: &Vec3, u: &Point2) -> (Vec3, F) {
        todo!()
    }
}

#[derive(Clone)]
pub struct MediumInterface {
    pub inside: Option<Arc<dyn Medium + Send + Sync>>,
    pub outside: Option<Arc<dyn Medium + Send + Sync>>,
}

impl MediumInterface {
    pub fn new_non_transition(medium: Option<Arc<dyn Medium + Send + Sync>>) -> Self {
        if let Some(med) = medium {
            Self {
                inside: Some(med.clone()),
                outside: Some(med.clone()),
            }
        } else {
            Self::new_empty()
        }
    }

    pub fn new_empty() -> Self {
        Self {
            inside: None,
            outside: None,
        }
    }

    // pub fn is_transition(&self) -> bool { 
    //     if let Some(ref inside) = self.inside {
    //         if let Some(ref outside) = self.outside {
    //             !Arc::ptr_eq(inside, outside)
    //         }
    //         else { false }
    //     } else { false }
    // }
}

pub trait Medium {
    fn transmittance(&self, ray: &Ray, rng: &RngGen) -> Color3;
    // fn sample(&self, ray: &ray, rng: &RngGen) -> 
}
