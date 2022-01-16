use crate::color::Color3;
use crate::common::*;
use crate::interaction::Interaction;
pub trait ScalarTexture {
    fn eval(&self, inter: &Interaction) -> F;
}

pub struct ConstantValue {
    pub val: F,
}

impl ScalarTexture for ConstantValue {
    fn eval(&self, inter: &Interaction) -> F {
        self.val
    }
}

pub trait ColorTexture {
    fn eval(&self, inter: &Interaction) -> Color3;
}

pub struct SolidColor {
    pub color: Color3,
}

impl ColorTexture for SolidColor {
    fn eval(&self, inter: &Interaction) -> Color3 {
        self.color
    }
}
