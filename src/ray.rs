use crate::common::*;
use crate::vector::*;

/// A simulated ray of light.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,

    pub t_min: F,
    pub t_max: F,
    pub time: F,

    pub has_differentials: bool,
    pub rx_origin: Option<Point3>,
    pub ry_origin: Option<Point3>,
    pub rx_direction: Option<Vec3>,
    pub ry_direction: Option<Vec3>,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: point3(0.0,0.0,0.0),
            direction: Vec3::zeros(),
            t_min: 0.0,
            t_max: F::INFINITY,
            time: 0.0,

            has_differentials: false,
            rx_origin: None,
            ry_origin: None,
            rx_direction: None,
            ry_direction: None,
        }
    }
}

impl Ray {
    pub fn new_non_differential(origin: Point3, direction: Vec3, t_min: F, t_max: F, time: F) -> Self {
        Self {
            origin, direction, t_min, t_max, time,
            has_differentials: false,
            ..Default::default()
        }
    }

    /// Computes the location given a distance along the ray.
    pub fn at(self, t: F) -> Point3 { self.origin + self.direction * t }

    pub fn scale_differentials(self, s: F) -> Option<Ray> {
        if !self.has_differentials { return None; }
        Some(Ray {
            rx_origin: Some(self.origin + (self.rx_origin.unwrap() - self.origin ) * s),
            ry_origin: Some(self.origin + (self.ry_origin.unwrap() - self.origin ) * s),
            rx_direction: Some(self.direction + (self.rx_direction.unwrap() - self.direction ) * s),
            ry_direction: Some(self.direction + (self.ry_direction.unwrap() - self.direction ) * s),
            ..Ray::from(self)
        })
    }
}
