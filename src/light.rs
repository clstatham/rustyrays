use std::{f64::consts::PI, sync::Arc};

use crate::{
    aabb::AABB3,
    color::{black, color3, Color3},
    common::{F, S},
    interaction::Interaction,
    ray::Ray,
    distributions::Distribution2D,
    scene::Scene,
    transform::Transform,
    vector::{point2, point3, spherical_phi, spherical_theta, vec3, Point2, Point3, Vec3},
};

pub struct VisibilityTester {
    pub p0: Arc<Interaction>,
    pub p1: Arc<Interaction>,
}

impl VisibilityTester {
    pub fn unoccluded(&self, scene: &Scene) -> bool {
        !scene.intersect_p(&self.p0.spawn_ray_to(&self.p1))
    }

    /// TODO: implement for translucent materials
    pub fn transmittance(&self, scene: &Scene) -> Color3 {
        // let mut ray = self.p0.spawn_ray_to(&self.p1);
        // let mut tr = color3(1.0,1.0,1.0);
        // loop { // TODO: this might need fixing
        //     if let Some(inter) = scene.intersect(&mut ray) {
        //         if let Some(prim) = inter.primitive {
        //             if let Some(_) = prim.material {
        //                 return black();
        //             }
        //         }
        //         ray = inter.spawn_ray_to(&&self.p1);
        //     } else {
        //         return tr;
        //     }
        // }
        black()
    }
}

pub struct LiResult {
    pub col: Color3,
    pub vis: VisibilityTester,
    pub wi: Vec3,
    pub pdf: F,
}

pub trait Light {
    fn num_samples(&self) -> S {
        1
    }
    fn light_to_world(&self) -> Transform;
    fn maybe_set_bounds(&mut self, world_bounds: &AABB3) {}
    fn sample_li(&self, inter: Arc<Interaction>, u: Point2) -> Option<LiResult>;
    fn pdf_li(&self, inter: &Interaction, w: &Vec3) -> F {
        0.0
    }
    fn power(&self) -> Color3;
    fn brightness(&self) -> F;
    fn le(&self, ray: &Ray) -> Color3 {
        black()
    }
}

pub struct PointLight {
    light_to_world: Transform,
    position: Point3,
    intensity: Color3,
    brightness: F,
}

impl PointLight {
    pub fn new(light_to_world: Transform, intensity: Color3, brightness: F) -> Self {
        Self {
            light_to_world,
            intensity,
            position: light_to_world.fpt(point3(0.0, 0.0, 0.0)),
            brightness,
        }
    }
}

impl Light for PointLight {
    fn light_to_world(&self) -> Transform {
        self.light_to_world
    }

    fn sample_li(&self, inter: Arc<Interaction>, u: Point2) -> Option<LiResult> {
        let wi = (self.position - inter.p).normalize();
        let pdf = 1.0;
        let vis = VisibilityTester {
            p0: inter.clone(),
            p1: Arc::new(Interaction::new_general(self.position, inter.time)),
        };
        let col = self.intensity * self.brightness / (inter.p - self.position).magnitude_squared();
        Some(LiResult { col, vis, wi, pdf })
    }

    fn power(&self) -> Color3 {
        4.0 * PI * self.intensity
    }

    fn brightness(&self) -> F {
        self.brightness
    }
}

pub struct ConstantInfiniteLight {
    light_to_world: Transform,
    intensity: Color3,
    world_center: Option<Point3>,
    world_radius: Option<F>,
    distr: Distribution2D,
    brightness: F,
}

impl ConstantInfiniteLight {
    pub fn new(light_to_world: Transform, intensity: Color3, brightness: F) -> Self {
        Self {
            light_to_world,
            intensity,
            world_center: None,
            world_radius: None,
            distr: Distribution2D::new(&[&[1.0]]),
            brightness,
        }
    }
}

impl Light for ConstantInfiniteLight {
    fn light_to_world(&self) -> Transform {
        self.light_to_world
    }

    fn le(&self, ray: &Ray) -> Color3 {
        // let w = self.light_to_world.ivec(ray.d).normalize();
        self.intensity
    }

    fn maybe_set_bounds(&mut self, world_bounds: &AABB3) {
        let sphere = world_bounds.bounding_sphere();
        self.world_center = Some(sphere.0);
        self.world_radius = Some(sphere.1);
        // ConstantInfiniteLight {
        //     world_center: Some(sphere.0),
        //     world_radius: Some(sphere.1),
        //     ..*self
        // }
    }

    fn power(&self) -> Color3 {
        if let Some(radius) = self.world_radius {
            let pir2 = PI * radius * radius;
            color3(
                self.intensity.x * pir2 * self.brightness,
                self.intensity.y * pir2 * self.brightness,
                self.intensity.z * pir2 * self.brightness,
            )
        } else {
            panic!("Uninitialized ConstantInfiniteLight is trying to be used! Did you call light.preprocess()?")
        }
    }

    fn sample_li(&self, inter: Arc<Interaction>, uv: Point2) -> Option<LiResult> {
        if let Some(radius) = self.world_radius {
            if let Some((uv, map_pdf)) = self.distr.sample_continuous(&uv) {
                if map_pdf == 0.0 {
                    return None;
                }
                let theta = uv[1] * PI;
                let phi = uv[0] * 2.0 * PI;
                let cos_theta = theta.cos();
                let sin_theta = theta.sin();
                let cos_phi = phi.cos();
                let sin_phi = phi.sin();
                let wi = self.light_to_world.fvec(&vec3(
                    sin_theta * cos_phi,
                    sin_theta * sin_phi,
                    cos_theta,
                ));
                let pdf;
                if sin_theta == 0.0 {
                    pdf = 0.0
                } else {
                    pdf = map_pdf / (2.0 * PI * PI * sin_theta);
                }
                let vis = VisibilityTester {
                    p0: inter.clone(),
                    p1: Arc::new(Interaction::new_general(
                        inter.p + wi * (2.0 * radius),
                        inter.time,
                    )),
                };
                Some(LiResult {
                    col: self.intensity * self.brightness,
                    wi,
                    pdf,
                    vis,
                })
            } else {
                None
            }
        } else {
            panic!("Uninitialized ConstantInfiniteLight is trying to be used! Did you call light.preprocess()?")
        }
    }

    fn pdf_li(&self, inter: &Interaction, w: &Vec3) -> F {
        let wi = self.light_to_world.ivec(w);
        let theta = spherical_theta(&wi);
        let phi = spherical_phi(&wi);
        let sin_theta = theta.sin();
        if sin_theta == 0.0 {
            0.0
        } else {
            self.distr
                .pdf(point2(phi / (2.0 * PI), theta / (2.0 * PI)) / (2.0 * PI * PI * sin_theta))
        }
    }

    fn brightness(&self) -> F {
        self.brightness
    }
}
