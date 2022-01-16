use std::sync::Arc;

extern crate tobj;

use rand::prelude::SliceRandom;
use tobj::LoadOptions;

use crate::aabb::AABB3;
use crate::common::*;
use crate::interaction::Shading;
use crate::interaction::Interaction;
use crate::media::MediumInterface;
use crate::ray::Ray;
use crate::distributions::Distribution1D;
use crate::shape::*;
use crate::transform::Transform;
use crate::vector::*;

#[derive(Clone)]
pub struct Triangle {
    shape_data: Arc<ShapeData>,
    pub a: S,
    pub b: S,
    pub c: S,
    pub positions: Arc<Vec<Point3>>,
    pub normals: Arc<Vec<Normal3>>,
    // pub texcoords: Arc<Vec<Point3>>,
}

impl Shape for Triangle {
    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn object_bound(&self) -> AABB3 {
        AABB3::new(self.positions[self.a], self.positions[self.b]).union(self.positions[self.c])
    }

    fn area(&self) -> F {
        let pa = &self.positions[self.a];
        let pb = &self.positions[self.a];
        let pc = &self.positions[self.a];
        let side1 = *pa - *pb;
        let side2 = *pc - *pa;
        let side3 = *pb - *pc;
        F::abs((pa.x * side3.y + pb.x * side2.y + pc.x * side1.y) / 2.0)
    }

    // Most of this code is from Twinklebear@Github's implementation at https://github.com/Twinklebear/tray_rust/blob/master/src/geometry/mesh.rs
    fn intersect(&self, ray: &mut Ray, _test_alpha_texture: bool) -> Option<Interaction> {
        let pa = &self.positions[self.a];
        let pb = &self.positions[self.b];
        let pc = &self.positions[self.c];
        let na = &self.normals[self.a];
        let nb = &self.normals[self.b];
        let nc = &self.normals[self.c];

        let e = [*pb - *pa, *pc - *pa];
        let mut s = [vec3(0.0, 0.0, 0.0); 2];
        s[0] = ray.direction.cross(&e[1]);
        let div = match s[0].dot(&e[0]) {
            d if d == 0.0 => return None,
            d => 1.0 / d,
        };

        let d = ray.origin - *pa;
        let mut bary = [0.0; 3];
        bary[1] = d.dot(&s[0]) * div;
        if bary[1] < 0.0 || bary[1] > 1.0 {
            return None;
        }

        s[1] = d.cross(&e[0]);
        bary[2] = ray.direction.dot(&s[1]) * div;
        if bary[2] < 0.0 || bary[1] + bary[2] > 1.0 {
            return None;
        }

        let t = e[1].dot(&s[1]) * div;
        if t < ray.t_min || t > ray.t_max {
            return None;
        }

        bary[0] = 1.0 - bary[1] - bary[2];
        ray.t_max = t;
        let p = ray.origin + ray.direction * t;

        let n = (bary[0] * *na + bary[1] * *nb + bary[2] * *nc).normalize();
        let texcoord = point2(0.0, 0.0); // TODO: add actual textures
        // let mi;
        // if self.shape_data.medium_interface.is_transition() { mi = self.shape_data.medium_interface; }
        // else { mi = MediumInterface::new_non_transition(ray.medium) }
        Some(Interaction::new_with_normal(
            p,
            -ray.direction,
            texcoord,
            n,
            ray.time,
            None,
            // mi,
        ))
    }

    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        let pa = &self.positions[self.a];
        let pb = &self.positions[self.b];
        let pc = &self.positions[self.c];
        // let na = &self.normals[self.a];
        // let nb = &self.normals[self.b];
        // let nc = &self.normals[self.c];

        let e = [*pb - *pa, *pc - *pa];
        let mut s = [vec3(0.0, 0.0, 0.0); 2];
        s[0] = ray.direction.cross(&e[1]);
        let div = match s[0].dot(&e[0]) {
            d if d == 0.0 => return false,
            d => 1.0 / d,
        };

        let d = ray.origin - *pa;
        let mut bary = [0.0; 3];
        bary[1] = d.dot(&s[0]) * div;
        if bary[1] < 0.0 || bary[1] > 1.0 {
            return false;
        }

        s[1] = d.cross(&e[0]);
        bary[2] = ray.direction.dot(&s[1]) * div;
        if bary[2] < 0.0 || bary[1] + bary[2] > 1.0 {
            return false;
        }

        let t = e[1].dot(&s[1]) * div;
        if t < 0.0 || t > ray.t_max {
            return false;
        }
        true
    }

    fn sample_u(&self, u: &Point2) -> Interaction {
        let b = Distribution1D::uniform_sample_triangle(u);
        let p0 = &self.positions[self.a];
        let p1 = &self.positions[self.b];
        let p2 = &self.positions[self.c];
        let n0 = &self.normals[self.a];
        let n1 = &self.normals[self.b];
        let n2 = &self.normals[self.c];
        let p = b[0] * p0 + b[1] * p1 + (1.0 - b[0] - b[1]) * p2;
        let mut n = (b[0] * n0 + b[1] * n1 + (1.0 - b[0] - b[1]) * n2).normalize();
        if self.shape_data.reverse_orientation {
            n *= -1.0;
        }
        let mut out = Interaction::new_general(p, 0.0);
        out.p = p;
        out.n = Some(n);
        out.shading = Some(Shading {
            n,
            ..Default::default()
        });
        out
    }
}

#[derive(Clone)]
pub struct Mesh {
    shape_data: Arc<ShapeData>,
    bvh: Vec<Triangle>,
}

impl Mesh {
    pub fn new(
        reverse_orientation: bool,
        positions: Arc<Vec<Point3>>,
        normals: Arc<Vec<Normal3>>,
        indices: Vec<UI>,
        object_to_world: Transform,
        medium_interface: MediumInterface,
    ) -> Self {
        let shape_data = Arc::new(ShapeData {
            reverse_orientation,
            transform_swaps_handedness: false,
            object_to_world,
            medium_interface,
        });
        let triangles = indices
            .chunks(3)
            .map(|i| Triangle {
                a: i[0] as S,
                b: i[1] as S,
                c: i[2] as S,
                positions: positions.clone(),
                normals: normals.clone(),
                shape_data: shape_data.clone(),
            })
            .collect();

        Self {
            shape_data,
            bvh: triangles,
        }
    }

    pub fn load_obj(path: String, object_to_world: Transform, medium_interface: MediumInterface) -> Option<Arc<Mesh>> {
        match tobj::load_obj(
            &path,
            &LoadOptions {
                single_index: true,
                triangulate: true,
                ignore_points: true,
                ignore_lines: true,
            },
        ) {
            Ok((models, _)) => {
                let mesh = &models[0].mesh;
                if mesh.normals.is_empty() {
                    eprintln!("ERROR: Mesh has no normals!");
                    return None;
                }
                println!(
                    "First mesh of {} has {} triangles.",
                    path,
                    mesh.indices.len() / 3
                );
                let positions = Arc::new(
                    mesh.positions
                        .chunks(3)
                        .map(|i| point3(i[0], i[1], i[2]))
                        .collect(),
                );
                let normals = Arc::new(
                    mesh.normals
                        .chunks(3)
                        .map(|i| normal3(i[0], i[1], i[2]))
                        .collect(),
                );
                Some(Arc::new(Mesh::new(
                    false,
                    positions,
                    normals,
                    mesh.indices.clone(),
                    object_to_world,
                    medium_interface,
                )))
            }
            Err(e) => {
                eprintln!("Failed to load {} due to {:?}", path, e);
                None
            }
        }
    }
}

impl Shape for Mesh {
    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn object_bound(&self) -> AABB3 {
        // AABB3::new(
        //     point3(-self.radius, -self.radius, self.z_min),
        //     point3(self.radius, self.radius, self.z_max),
        // )
        let mut aabb = AABB3::default();
        for triangle in self.bvh.iter() {
            aabb = aabb.combine(triangle.object_bound());
        }
        aabb
    }

    fn intersect(&self, ray: &mut Ray, test_alpha_texture: bool) -> Option<Interaction> {
        // let mut ray = self.shape_data.obj_to_world.iray(r);
        let mut result = None;
        // let inv_dir = vec3(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
        // let neg_dir = [(ray.direction.x < 0.0) as S, (ray.direction.y < 0.0) as S, (ray.direction.z < 0.0) as S];
        for node in self.bvh.iter() {
            result = node.intersect(ray, test_alpha_texture).or(result);
        }
        if let Some(mut inter) = result {
            inter.create_bsdf();
            return Some(inter);
        }
        None
    }

    fn area(&self) -> F {
        let mut total = 0.0;
        for obj in self.bvh.iter() {
            total += obj.area();
        }
        total
    }

    fn intersect_p(&self, r: &Ray, test_alpha_texture: bool) -> bool {
        // let ray = &self.shape_data.obj_to_world.iray(r);
        for node in self.bvh.iter() {
            if node.intersect_p(r, test_alpha_texture) {
                return true;
            }
        }
        false
    }

    fn sample_u(&self, u: &Point2) -> Interaction {
        let tri = self.bvh.choose(&mut rand::thread_rng()).unwrap();
        tri.sample_u(u)
    }
}
