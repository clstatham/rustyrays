use std::f32::consts::PI;

use crate::aabb::AABB3;
use crate::common::*;
use crate::interaction::SurfaceInteraction;
use crate::matrix::*;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::vector::*;

#[derive(Debug, Clone, Copy)]
pub struct ShapeData {
    // pub obj_to_world: Transform,
    pub reverse_orientation: bool,
    pub transform_swaps_handedness: bool,
    pub object_to_world: Transform,
}

pub trait Shape {
    // fn new(obj_to_world: Transform, reverse_orientation: bool) -> Self;
    fn shape_data(&self) -> &ShapeData;
    // fn obj_to_world(&self) -> &Transform { &self.shape_data().obj_to_world }
    // fn world_to_obj(&self) -> Transform { self.shape_data().obj_to_world.as_inverse_transform() }
    fn object_bound(&self) -> AABB3;
    // fn world_bound(&self) -> AABB3 { &self.object_bound() }
    fn intersect(&self, _r: &mut Ray, _test_alpha_texture: bool) -> Option<SurfaceInteraction>;
    fn intersect_p(&self, r: &Ray, test_alpha_texture: bool) -> bool;
    fn area(&self) -> F;
    fn pdf(&self, _ref: SurfaceInteraction) -> F {
        1.0 / self.area()
    }
    fn pdf_wi(&self, inter: &SurfaceInteraction, wi: &Vec3) -> F {
        let mut ray = inter.spawn_ray_to_point(wi);
        if let Some(isect_light) = self.intersect(&mut ray, false) {
            (inter.p - isect_light.p).magnitude_squared()
                / (isect_light.n.unwrap().dot(&-wi).abs() * self.area())
        } else {
            0.0
        }
    }
    fn sample_u(&self, u: &Point2) -> SurfaceInteraction;
    fn sample_inter(&self, inter: &SurfaceInteraction, u: &Point2) -> SurfaceInteraction {
        self.sample_u(u)
    }
}
