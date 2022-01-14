use std::mem::swap;
use std::ops::Add;

extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
use crate::common::*;
use crate::ray::Ray;
use crate::vector::*;

/// An axis aligned bounding box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AABB3 {
    pub p_min: Point3,
    pub p_max: Point3,
}

impl AABB3 {
    pub fn new(p1: Point3, p2: Point3) -> Self {
        Self {
            p_min: point3(p1.x.min(p2.x), p1.y.min(p2.y), p1.z.min(p2.z)),
            p_max: point3(p1.x.max(p2.x), p1.y.max(p2.y), p1.z.max(p2.z)),
        }
    }

    /// Constructs a new AABB containing both the AABB and the given point.
    pub fn union(self, p: Point3) -> AABB3 {
        AABB3 {
            p_min: point3(
                self.p_min.x.min(p.x),
                self.p_min.y.min(p.y),
                self.p_min.z.min(p.z),
            ),
            p_max: point3(
                self.p_max.x.max(p.x),
                self.p_max.y.max(p.y),
                self.p_max.z.max(p.z),
            ),
        }
    }

    /// Constructs a new AABB containing both AABBs.
    pub fn combine(self, other: AABB3) -> AABB3 {
        AABB3 {
            p_min: point3(
                self.p_min.x.min(other.p_min.x),
                self.p_min.y.min(other.p_min.y),
                self.p_min.z.min(other.p_min.z),
            ),
            p_max: point3(
                self.p_max.x.max(other.p_max.x),
                self.p_max.y.max(other.p_max.y),
                self.p_max.z.max(other.p_max.z),
            ),
        }
    }

    /// Constructs a new AABB containing the intersection of two AABBs.
    pub fn intersect(self, other: AABB3) -> AABB3 {
        AABB3 {
            p_min: point3(
                self.p_min.x.max(other.p_min.x),
                self.p_min.y.max(other.p_min.y),
                self.p_min.z.max(other.p_min.z),
            ),
            p_max: point3(
                self.p_max.x.min(other.p_max.x),
                self.p_max.y.min(other.p_max.y),
                self.p_max.z.min(other.p_max.z),
            ),
        }
    }

    pub fn intersect_p(self, ray: Ray) -> (Option<F>, Option<F>) {
        let mut t0 = 0.0;
        let mut t1 = ray.t_max;
        for i in 0..3 {
            let mut inv_ray_dir = 1.0 / ray.direction[i];
            if inv_ray_dir.is_nan() {
                inv_ray_dir = ray.direction[i].signum() * F::INFINITY;
            } // convert NaN to +/- infinity so the calculation still works
            let mut t_near = (self.p_min[i] - ray.origin[i]) * inv_ray_dir;
            let mut t_far = (self.p_max[i] - ray.origin[i]) * inv_ray_dir;
            if t_near > t_far {
                swap(&mut t_near, &mut t_far)
            }
            t_far *= 1.0 + 2.0 * gamma(3.0);
            if t_near > t0 {
                t0 = t_near;
            }
            if t_far < t1 {
                t1 = t_far;
            }
            if t0 > t1 {
                return (None, None);
            }
        }
        (Some(t0), Some(t1))
    }

    /// Returns whether two AABBs overlap.
    pub fn overlaps(self, other: AABB3) -> bool {
        let x = (self.p_max.x >= other.p_max.x) && (self.p_min.x <= other.p_min.x);
        let y = (self.p_max.y >= other.p_max.y) && (self.p_min.y <= other.p_min.y);
        let z = (self.p_max.z >= other.p_max.z) && (self.p_min.z <= other.p_min.z);
        x && y && z
    }

    /// Returns whether the given point is inside the AABB.
    pub fn inside(self, p: Point3) -> bool {
        p.x >= self.p_min.x
            && p.x <= self.p_max.x
            && p.y >= self.p_min.y
            && p.y <= self.p_max.y
            && p.z >= self.p_min.z
            && p.z <= self.p_max.z
    }

    /// Scales the AABB by a linear factor on all sides equally.
    pub fn expand(self, delta: F) -> AABB3 {
        AABB3 {
            p_min: self.p_min - vec3(delta, delta, delta),
            p_max: self.p_max + vec3(delta, delta, delta),
        }
    }

    /// Returns the vector from the minimum point of the AABB to the maximum point.
    pub fn diagonal(self) -> Vec3 {
        self.p_max - self.p_min
    }

    /// Returns the AABB's surface area.
    pub fn surface_area(self) -> F {
        let d = self.diagonal();
        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    /// Returns the AABB's volume.
    pub fn volume(self) -> F {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    /// Returns the axis index of the longest dimension of the AABB.
    pub fn max_extent(self) -> UI {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            X_AXIS
        } else if d.y > d.z {
            Y_AXIS
        } else {
            Z_AXIS
        }
    }

    /// Linearly interpolates two AABBs by the parameter 0 <= t <= 1.
    pub fn lerp(self, t: Point3) -> Point3 {
        point3(
            lerp(t.x, self.p_min.x, self.p_max.x),
            lerp(t.y, self.p_min.y, self.p_max.y),
            lerp(t.z, self.p_min.z, self.p_max.z),
        )
    }

    /// ??? TODO: Remember what this does.
    pub fn offset(self, p: Point3) -> Vec3 {
        let mut o = p - self.p_min;
        if self.p_max.x > self.p_min.x {
            o.x /= self.p_max.x - self.p_min.x;
        }
        if self.p_max.y > self.p_min.y {
            o.y /= self.p_max.y - self.p_min.y;
        }
        if self.p_max.z > self.p_min.z {
            o.z /= self.p_max.z - self.p_min.z;
        }
        o
    }

    /// Returns the centerpoint and radius of a sphere that would fully contain this AABB.
    pub fn bounding_sphere(self) -> (Point3, F) {
        let center = (self.p_min.add(self.p_max)) / 2.0;
        let radius = if self.inside(center) {
            distance3(&center, &self.p_max)
        } else {
            0.0
        };
        (center, radius)
    }
}

impl Default for AABB3 {
    /// Constructs an infinitely large AABB. Be careful!
    fn default() -> Self {
        Self {
            p_min: point3(-F::INFINITY, -F::INFINITY, -F::INFINITY),
            p_max: point3(F::INFINITY, F::INFINITY, F::INFINITY),
        }
    }
}

impl From<Point3> for AABB3 {
    fn from(p: Point3) -> Self {
        Self { p_min: p, p_max: p }
    }
}
