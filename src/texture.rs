use crate::color::Color3;
use crate::common::*;
use crate::interaction::SurfaceInteraction;
use crate::rng::RngGen;
use crate::vector::*;

pub trait ScalarTexture {
    fn eval(&self, inter: &SurfaceInteraction) -> F;
}

pub struct ConstantValue {
    pub val: F,
}

impl ScalarTexture for ConstantValue {
    fn eval(&self, inter: &SurfaceInteraction) -> F {
        self.val
    }
}

pub trait ColorTexture {
    fn eval(&self, inter: &SurfaceInteraction) -> Color3;
}

pub struct SolidColor {
    pub color: Color3,
}

impl ColorTexture for SolidColor {
    fn eval(&self, inter: &SurfaceInteraction) -> Color3 {
        self.color
    }
}
