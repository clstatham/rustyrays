use std::rc::Rc;

use crate::{
    interaction::SurfaceInteraction, light::Light, primitive::Primitive, ray::Ray, shape::Shape, aabb::AABB3,
};

pub struct Scene {
    pub objs: Vec<Primitive>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn add(&mut self, obj: Primitive) {
        self.objs.push(obj);
    }

    pub fn preprocess(&mut self) {
        let bounds = self.world_bounds();
        for light in self.lights.iter_mut() {
            // light.preprocess(Box::new(&mut self));
            light.maybe_set_bounds(&bounds);
        }
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction> {
        let mut result = None;
        for node in self.objs.iter() {
            result = node.intersect(ray, false).or(result);
        }
        result
    }

    pub fn intersect_p(&self, ray: &Ray) -> bool {
        for node in self.objs.iter() {
            if node.intersect_p(ray, false) {
                return true;
            }
        }
        false
    }

    pub fn world_bounds(&self) -> AABB3 {
        let mut bounds = AABB3::default();
        for node in self.objs.iter() {
            bounds = bounds.combine(node.object_bound());
        }
        bounds
    }
}
