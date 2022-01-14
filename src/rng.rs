use rand_distr::{Uniform, Distribution};

use crate::common::F;
use crate::vector::*;


pub struct RngGen {
    uniform: Uniform<F>,
}

impl RngGen {
    pub fn new() -> Self { Self { uniform: Uniform::new(0.0, 1.0) } }

    pub fn sample_0_1(&self) -> F { self.uniform.sample(&mut rand::thread_rng()) }
    pub fn sample_neg1_1(&self) -> F { self.uniform.sample(&mut rand::thread_rng()) * 2.0 - 1.0 }
    pub fn uniform_sample_point2(&self) -> Point2 { point2(self.sample_0_1(), self.sample_0_1()) }
}