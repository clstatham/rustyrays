use std::sync::Arc;

use crate::aabb::AABB3;
use crate::common::*;
use crate::interaction::Interaction;
use crate::light::Light;
use crate::material::*;
use crate::ray::Ray;
use crate::rng::RngGen;
use crate::shape::*;

#[derive(Clone)]
pub struct Primitive {
    pub shape: Arc<dyn Shape + Send + Sync>,
    pub material: Arc<dyn Material + Send + Sync>,
    // TODO: add material, light properties
    pub light: Option<Arc<dyn Light + Send + Sync>>,
}

impl Primitive {
    pub fn new(
        shape: Arc<dyn Shape + Send + Sync>,
        material: Arc<dyn Material + Send + Sync>,
        light: Option<Arc<dyn Light + Send + Sync>>,
    ) -> Self {
        Self {
            shape,
            material,
            light,
        }
    }

    pub fn scatter(&self, inter: &mut Interaction, rng: &RngGen) {
        // if let Some(material) = self.material.clone() {
        // material.calculate_bsdf(inter, rng);
        // }
    }
}
impl Shape for Primitive {
    fn intersect(&self, ray: &mut Ray, test_alpha_texture: bool) -> Option<Interaction> {
        let mut transformed_ray = self.shape.shape_data().object_to_world.iray(ray);
        match self
            .shape
            .intersect(&mut transformed_ray, test_alpha_texture)
        {
            Some(mut inter) => {
                ray.t_max = transformed_ray.t_max;
                inter.primitive = Some(Arc::new(self.clone()));
                self.material.calculate_bsdf(&mut inter);
                Some(
                    self.shape
                        .shape_data()
                        .object_to_world
                        .forward_surface_interaction_transform(inter),
                )
            }
            None => None,
        }
    }

    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        let transformed_ray = self.shape.shape_data().object_to_world.iray(ray);
        self.shape.intersect_p(&transformed_ray, test_alpha_texture)
    }

    fn shape_data(&self) -> &ShapeData {
        self.shape.shape_data()
    }

    fn object_bound(&self) -> AABB3 {
        self.shape
            .shape_data()
            .object_to_world
            .faabb(&self.shape.object_bound())
    }

    fn area(&self) -> F {
        self.shape.area()
    }

    fn sample_u(&self, u: &crate::vector::Point2) -> Interaction {
        self.shape.sample_u(u)
    }

    fn sample_inter(&self, inter: &Interaction, u: &crate::vector::Point2) -> Interaction {
        self.shape.sample_inter(inter, u)
    }
}
