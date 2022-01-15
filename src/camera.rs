use crate::ray::Ray;
use crate::transform::Transform;
use crate::vector::*;
use crate::{common::*, ASPECT_RATIO, HEIGHT, WIDTH};

pub struct SimpleCamera {
    pub fov: F,
    // pub origin: Point3,
    lookat: Transform,
}

impl SimpleCamera {
    pub fn new(origin: Point3, lookat: Point3, fov: F) -> Self {
        Self {
            fov,
            lookat: Transform::new_lookat(origin, lookat, vec3(0.0, 1.0, 0.0)),
        }
    }

    pub fn get_ray(&self, xy: Point2) -> Ray {
        let angle = deg2rad(self.fov / 2.0).tan();
        let xx = (2.0 * ((xy.x + 0.5) * (1.0 / WIDTH as F)) - 1.0) * angle * ASPECT_RATIO;
        let yy = (1.0 - 2.0 * ((xy.y + 0.5) * (1.0 / HEIGHT as F))) * angle;
        let direction = vec3(xx, yy, -1.0).normalize();
        let ray =
            Ray::new_non_differential(point3(0.0, 0.0, 0.0), direction, 0.0001, F::INFINITY, 0.0);
        self.lookat.iray(&ray)
    }
}
