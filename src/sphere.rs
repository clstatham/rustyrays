use std::f64::consts::PI;

use crate::aabb::AABB3;
use crate::common::*;
use crate::interaction::Interaction;
use crate::media::MediumInterface;
use crate::ray::Ray;
use crate::distributions::Distribution1D;
use crate::shape::*;
use crate::transform::Transform;
use crate::vector::*;

#[derive(Clone)]
pub struct Sphere {
    shape_data: ShapeData,
    radius: F,
    // z_min: F,
    // z_max: F,
    // phi_max: F,
}

impl Sphere {
    pub fn new(reverse_orientation: bool, radius: F, object_to_world: Transform, medium_interface: MediumInterface) -> Self {
        Self {
            shape_data: ShapeData {
                reverse_orientation,
                transform_swaps_handedness: false,
                object_to_world,
                medium_interface,
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

    fn intersect(&self, ray: &mut Ray, test_alpha_texture: bool) -> Option<Interaction> {
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
        let mut p = ray.origin + ray.direction * t_shape_hit;
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
        // let mi;
        // if self.shape_data.medium_interface.is_transition() { mi = self.shape_data.medium_interface; }
        // else { mi = MediumInterface::new_non_transition(ray.medium) }
        Some(Interaction::new(
            p,
            -ray.direction,
            point2(u, v),
            dpdu,
            dpdv,
            time,
            None,
            None,
            // mi,
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

    fn sample_u(&self, u: &Point2) -> Interaction {
        let mut p_obj = self.radius * Distribution1D::uniform_sample_sphere(u);
        let mut n = self
            .shape_data
            .object_to_world
            .fnorm(&normal3(p_obj.x, p_obj.y, p_obj.z))
            .normalize();
        if self.shape_data.reverse_orientation {
            n *= -1.0;
        }
        p_obj *= self.radius / distance3d(&p_obj, &point3(0.0, 0.0, 0.0));
        p_obj = self.shape_data.object_to_world.fpt(p_obj);
        let mut out = Interaction::new_general(p_obj, 0.0);
        out.n = Some(n);
        out
    }
}
