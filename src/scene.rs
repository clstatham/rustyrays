use std::rc::Rc;

use crate::{
    interaction::SurfaceInteraction, light::Light, primitive::Primitive, ray::Ray, shape::Shape,
};

pub struct Scene {
    pub objs: Vec<Rc<Primitive>>,
    pub lights: Vec<Rc<dyn Light>>,
}

impl Scene {
    pub fn add(&mut self, obj: Rc<Primitive>) {
        self.objs.push(obj);
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
}
