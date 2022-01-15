use std::f32::consts::PI;
use std::rc::Rc;

use crate::aabb::AABB3;
use crate::common::*;
use crate::interaction::SurfaceInteraction;
use crate::ray::Ray;
use crate::shape::*;
use crate::transform::Transform;
use crate::vector::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    shape_data: ShapeData,
    radius: F,
    // z_min: F,
    // z_max: F,
    // phi_max: F,
}

impl Sphere {
    pub fn new(reverse_orientation: bool, radius: F) -> Self {
        Self {
            shape_data: ShapeData {
                reverse_orientation,
                transform_swaps_handedness: false,
            },
            radius,
            // z_min: z_min.min(z_max).clamp(-radius, radius),
            // z_max: z_min.max(z_max).clamp(-radius, radius),
            // theta_min: (z_min / radius).clamp(-1.0, 1.0).acos(),
            // theta_max: (z_max / radius).clamp(-1.0, 1.0).acos(),
            // phi_max: phi_max.clamp(0.0, 2.0 * PI),
        }
    }
}

impl Shape for Sphere {
    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn object_bound(&self) -> AABB3 {
        AABB3::new(
            point3(-self.radius, -self.radius, -self.radius),
            point3(self.radius, self.radius, self.radius),
        )
    }

    fn intersect(&self, ray: &mut Ray, test_alpha_texture: bool) -> Option<SurfaceInteraction> {
        // let mut ray = self.shape_data.obj_to_world.iray(r);
        let time = ray.time;
        let a = ray.direction.magnitude_squared();
        let b = 2.0 * ray.direction.dot(&ray.origin);
        let c = ray.origin.dot(&ray.origin) - self.radius * self.radius;
        let (t0, t1) = match quadratic(a, b, c) {
            None => return None,
            Some((t0_, t1_)) => (t0_, t1_),
        };
        let mut t_shape_hit = t0;
        if t_shape_hit < ray.t_min || t_shape_hit > ray.t_max {
            t_shape_hit = t1;
            if t_shape_hit < ray.t_min || t_shape_hit > ray.t_max {
                return None;
            }
        }
        ray.t_max = t_shape_hit;
        let mut p = ray.at(t_shape_hit);
        if p.x == 0.0 && p.y == 0.0 {
            p.x = 1e-5 * self.radius;
        }
        let n = normal3(p.x, p.y, p.z);
        let theta = F::acos((p.z / self.radius).clamp(-1.0, 1.0));

        let inv_z = 1.0 / F::sqrt(p.x * p.x + p.y * p.y);
        let cos_phi = p.x * inv_z;
        let sin_phi = p.y * inv_z;
        let u = match F::atan2(p.x, p.y) / (2.0 * PI) {
            x if x < 0.0 => x + 1.0,
            x => x,
        };
        let v = theta / PI;
        let dpdu = vec3(-2.0 * PI * p.y, 2.0 * PI * p.x, 0.0);
        let dpdv = vec3(p.z * cos_phi, p.z * sin_phi, -self.radius * theta.sin()) * PI;
        Some(SurfaceInteraction::new(
            p,
            -ray.direction,
            point2(u, v),
            dpdu,
            dpdv,
            time,
            None,
            None,
        ))
    }

    fn area(&self) -> F {
        4.0 * self.radius * self.radius * PI
    }

    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        // let ray = &self.shape_data.obj_to_world.iray(r);
        // let time = ray.time;
        let a = ray.direction.magnitude_squared();
        let b = 2.0 * ray.direction.dot(&ray.origin);
        let c = ray.origin.magnitude_squared() - self.radius * self.radius;
        let (t0, t1) = match quadratic(a, b, c) {
            None => return false,
            Some((t0_, t1_)) => (t0_, t1_),
        };
        let mut t_shape_hit = t0;
        if t_shape_hit < ray.t_min || t_shape_hit > ray.t_max {
            t_shape_hit = t1;
            if t_shape_hit < ray.t_min || t_shape_hit > ray.t_max {
                return false;
            }
        }
        true
    }
}
