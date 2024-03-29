use std::sync::Arc;

use rand::prelude::IteratorRandom;

use crate::{
    camera::SimpleCamera,
    color::{black, color3, Color3},
    common::{F, S},
    interaction::Interaction,
    light::Light,
    material::{BXDF_ALL, BXDF_SPECULAR},
    ray::Ray,
    rng::RngGen,
    scene::Scene,
    vector::Point2,
};

pub trait Integrator {
    // fn render(&self, scene: &Scene, cam: &Camera) {
    //     self.preprocess(scene, cam);

    // }
    fn preprocess(&mut self, scene: &Scene, cam: &SimpleCamera) {}
    fn li(&self, ray: &mut Ray, scene: &Scene, depth: S, rng: &RngGen) -> Color3;
    // fn specular_reflect(&self, ray: &Ray, inter: &SurfaceInteraction, scene: &Scene, depth: S) -> Color3;
    // fn specular_transmit(&self, ray: &Ray, inter: &SurfaceInteraction, scene: &Scene, depth: S) -> Color3;
}

pub fn power_heuristic(nf: S, f_pdf: F, ng: S, g_pdf: F) -> F {
    let f = nf as F * f_pdf;
    let g = ng as F * g_pdf;
    (f * f) / (f * f + g * g)
}

#[allow(clippy::borrowed_box)]
pub fn estimate_direct(
    inter: &Interaction,
    u_scattering: Point2,
    light: &Box<dyn Light + Send + Sync>,
    u_light: Point2,
    scene: &Scene,
    rng: &RngGen,
) -> Color3 {
    let flags = BXDF_ALL & !BXDF_SPECULAR;
    let mut ld = black();
    if let Some(mut li) = light.sample_li(Arc::new(inter.clone()), u_light) {
        if li.pdf > 0.0 && li.col != black() {
            let f = inter
                .bsdf
                .as_ref()
                .unwrap()
                .f(&inter.wo.unwrap(), &li.wi, flags)
                * li.wi.dot(&inter.shading.as_ref().unwrap().n);
            let scattering_pdf =
                inter
                    .bsdf
                    .as_ref()
                    .unwrap()
                    .pdf(&inter.wo.unwrap(), &li.wi, flags);
            if f != black() {
                if !li.vis.unoccluded(scene) {
                    li.col = black();
                } else {
                    let weight = power_heuristic(1, li.pdf, 1, scattering_pdf);
                    ld += f.component_mul(&li.col) * weight / li.pdf;
                }
            }
        }
    }
    if let Some((mut f, scattering_pdf, wi, sampled_type)) =
        inter
            .bsdf
            .as_ref()
            .unwrap()
            .sample_f(&inter.wo.unwrap(), &u_scattering, flags)
    {
        f *= wi.dot(&inter.shading.as_ref().unwrap().n).abs();
        let sampled_specular = sampled_type & BXDF_SPECULAR != 0;
        if f != black() && scattering_pdf > 0.0 {
            let mut weight = 1.0;
            if !sampled_specular {
                let li_pdf = light.pdf_li(inter, &wi);
                if li_pdf == 0.0 {
                    return ld;
                }
                weight = power_heuristic(1, scattering_pdf, 1, li_pdf);
            }
            let ray = inter.spawn_ray_to_point(&wi);
            // let tr = 1.0;
            // let mut li = black();
            // if let Some(light_isect) = scene.intersect(&mut ray) {
            // li = light_isect.le(-wi);
            // } else {
            let li = light.le(&ray);
            // }
            if li != black() {
                ld += f.component_mul(&li) * weight / scattering_pdf;
            }
        }
    }
    ld
}

pub fn uniform_sample_all_lights(
    inter: &Interaction,
    scene: &Scene,
    n_light_samples: Vec<S>,
    rng: &RngGen,
) -> Color3 {
    let mut out_color = black();
    for (j, light) in scene.lights.iter().enumerate() {
        let n_samples = n_light_samples[j];
        let u_light_array = rng.get_2d_array(n_samples);
        let u_scattering_array = rng.get_2d_array(n_samples);
        if u_light_array.is_empty() || u_scattering_array.is_empty() {
            let u_light = rng.uniform_sample_point2();
            let u_scattering = rng.uniform_sample_point2();
            out_color += estimate_direct(inter, u_scattering, light, u_light, scene, rng);
        } else {
            let mut ld = black();
            for k in 0..n_samples {
                ld += estimate_direct(
                    inter,
                    u_scattering_array[k],
                    light,
                    u_light_array[k],
                    scene,
                    rng,
                );
            }
            out_color += ld / n_samples as F;
        }
    }
    out_color
}

pub fn uniform_sample_one_light(inter: &Interaction, scene: &Scene, rng: &RngGen) -> Color3 {
    if scene.lights.is_empty() {
        return black();
    }
    let light = scene.lights.iter().choose(&mut rand::thread_rng()).unwrap();
    let u_light = rng.uniform_sample_point2();
    let u_scattering = rng.uniform_sample_point2();
    scene.lights.len() as F * estimate_direct(inter, u_scattering, light, u_light, scene, rng)
}

#[derive(PartialEq)]
pub enum LightStrategy {
    UniformSampleAll,
    UniformSampleOne,
}

pub struct DirectLightingIntegrator {
    strategy: LightStrategy,
    max_depth: S,
    n_light_samples: Vec<S>,
}

impl DirectLightingIntegrator {
    pub fn new(strategy: LightStrategy, max_depth: S) -> Self {
        Self {
            strategy,
            max_depth,
            n_light_samples: vec![],
        }
    }
}

impl Integrator for DirectLightingIntegrator {
    fn preprocess(&mut self, scene: &Scene, cam: &SimpleCamera) {
        if self.strategy == LightStrategy::UniformSampleAll {
            for light in scene.lights.iter() {
                self.n_light_samples.push(light.num_samples());
            }
        }
    }

    fn li(&self, ray: &mut Ray, scene: &Scene, depth: S, rng: &RngGen) -> Color3 {
        // let mut rr_factor = 1.0;
        // if depth >= self.max_depth {
        //     let rr_stop_prob = 1.0f32.min(0.0625 * depth as F);
        //     if rng.sample_0_1() <= rr_stop_prob {
        //         return black();
        //     }
        //     rr_factor = 1.0 / (1.0 - rr_stop_prob);
        // }
        let mut out_color = black();
        if let Some(inter) = scene.intersect(ray) {
            if self.strategy == LightStrategy::UniformSampleAll {
                out_color +=
                    uniform_sample_all_lights(&inter, scene, self.n_light_samples.clone(), rng);
            } else {
                out_color += uniform_sample_one_light(&inter, scene, rng);
            }
            // if depth < self.max_depth {
            //     out_color += self.li(ray, scene, depth+1, rng);
            // }
        } else {
            return black();
        }
        out_color // * rr_factor
    }
}

pub struct PathIntegrator {
    max_depth: S,
}

impl PathIntegrator {
    pub fn new(max_depth: S) -> Self {
        Self { max_depth }
    }
}

impl Integrator for PathIntegrator {
    fn li(&self, original_ray: &mut Ray, scene: &Scene, depth: S, rng: &RngGen) -> Color3 {
        let mut out_color = black();
        let mut ray = original_ray.to_owned();
        let mut beta = color3(1.0, 1.0, 1.0);
        let mut specular_bounce = false;
        let mut bounces = 0;
        loop {
            let inter_opt = scene.intersect(&mut ray);
            if bounces == 0 || specular_bounce {
                if let Some(ref inter) = inter_opt {
                    // out_color += beta.component_mul(&inter.le(&-ray.direction));
                } else {
                    for light in scene.lights.iter() {
                        out_color += beta.component_mul(&light.le(&ray));
                    }
                }
            }
            if inter_opt.is_none() || bounces >= self.max_depth {
                break;
            }

            let mut inter = inter_opt.unwrap().clone();

            inter.scatter(&mut ray, rng);
            if inter.bsdf.is_none() {
                ray = inter.spawn_ray(ray.direction);
                bounces -= 1;
                continue;
            }

            let bsdf = inter.bsdf.as_ref().unwrap();
            out_color += beta.component_mul(&uniform_sample_one_light(&inter, scene, rng));
            let wo = -ray.direction;
            if let Some((f, pdf, wi, flags)) =
                bsdf.sample_f(&wo, &rng.uniform_sample_point2(), BXDF_ALL)
            {
                if f == black() || pdf == 0.0 {
                    break;
                }
                beta.component_mul_assign(
                    &(f * wi.dot(&inter.shading.as_ref().unwrap().n).abs() / pdf),
                );
                specular_bounce = flags & BXDF_SPECULAR != 0;
                ray = inter.spawn_ray(wi);
            }

            // TODO: Subsurface scattering here!

            if bounces > 3 {
                let q = F::max(0.05, 1.0 - beta.y);
                if rng.sample_0_1() < q {
                    break;
                }
                beta /= 1.0 - q;
            }

            bounces += 1;
        }
        out_color
    }
}
