use std::rc::Rc;

use crate::aabb::AABB3;
use crate::common::*;
use crate::interaction::SurfaceInteraction;
use crate::light::Light;
use crate::material::*;
use crate::ray::Ray;
use crate::rng::RngGen;
use crate::shape::*;
use crate::transform::Transform;

#[derive(Clone)]
pub struct Primitive {
    pub shape: Rc<dyn Shape>,
    pub material: Rc<dyn Material>,
    // TODO: add material, light properties
    pub light: Option<Rc<dyn Light>>,
    pub object_to_world: Transform,
}

impl Primitive {
    pub fn new(
        shape: Rc<dyn Shape>,
        object_to_world: Transform,
        material: Rc<dyn Material>,
        light: Option<Rc<dyn Light>>,
    ) -> Self {
        Self {
            shape,
            object_to_world,
            material,
            light,
        }
    }

    pub fn scatter(&self, inter: &mut SurfaceInteraction, rng: &RngGen) {
        // if let Some(material) = self.material.clone() {
            // material.calculate_bsdf(inter, rng);
        // }
    }
}
impl Shape for Primitive {
    fn intersect(&self, ray: &mut Ray, test_alpha_texture: bool) -> Option<SurfaceInteraction> {
        let mut transformed_ray = self.object_to_world.iray(ray);
        match self
            .shape
            .intersect(&mut transformed_ray, test_alpha_texture)
        {
            Some(mut inter) => {
                ray.t_max = transformed_ray.t_max;
                inter.primitive = Some(Rc::new(self.clone()));
                self.material.calculate_bsdf(&mut inter);
                Some(
                    self.object_to_world
                        .forward_surface_interaction_transform(inter),
                )
            }
            None => None,
        }
    }

    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        let transformed_ray = self.object_to_world.iray(ray);
        self.shape.intersect_p(&transformed_ray, test_alpha_texture)
    }

    fn shape_data(&self) -> &ShapeData {
        self.shape.shape_data()
    }

    fn object_bound(&self) -> AABB3 {
        self.object_to_world.faabb(&self.shape.object_bound())
    }

    fn area(&self) -> F {
        self.shape.area()
    }
}
