use std::{f32::consts::PI, rc::Rc};

use nalgebra::Norm;

use crate::{
    color::{black, color3, Color3},
    common::F,
    interaction::SurfaceInteraction,
    scene::Scene,
    transform::Transform,
    vector::{point3, Point2, Point3, Vec3},
};

pub struct VisibilityTester {
    pub p0: Rc<SurfaceInteraction>,
    pub p1: Rc<SurfaceInteraction>,
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
    pub wi: Option<Vec3>,
    pub pdf: Option<F>,
}

pub trait Light {
    fn light_to_world(&self) -> Transform;
    fn sample_li(&self, inter: Rc<SurfaceInteraction>, u: Point2) -> LiResult;
    fn power(&self) -> Color3;
}

pub struct PointLight {
    light_to_world: Transform,
    position: Point3,
    intensity: Color3,
}

impl PointLight {
    pub fn new(light_to_world: Transform, intensity: Color3) -> Self {
        Self {
            light_to_world,
            intensity,
            position: light_to_world.fpt(point3(0.0, 0.0, 0.0)),
        }
    }
}

impl Light for PointLight {
    fn light_to_world(&self) -> Transform {
        self.light_to_world
    }

    fn sample_li(&self, inter: Rc<SurfaceInteraction>, u: Point2) -> LiResult {
        let wi = (self.position - inter.p).normalize();
        let pdf = 1.0;
        let vis = VisibilityTester {
            p0: inter.clone(),
            p1: Rc::new(SurfaceInteraction::new_general(self.position, inter.time)),
        };
        let col = self.intensity / (inter.p - self.position).magnitude_squared();
        LiResult {
            col,
            vis,
            wi: Some(wi),
            pdf: Some(pdf),
        }
    }

    fn power(&self) -> Color3 {
        4.0 * PI * self.intensity
    }
}
