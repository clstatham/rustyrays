use rand_distr::{Distribution, Uniform};

use crate::common::{F, S};
use crate::vector::*;

pub struct RngGen {
    uniform: Uniform<F>,
}

impl RngGen {
    pub fn new() -> Self {
        Self {
            uniform: Uniform::new(0.0, 1.0),
        }
    }

    pub fn sample_0_1(&self) -> F {
        self.uniform.sample(&mut rand::thread_rng())
    }
    pub fn sample_neg1_1(&self) -> F {
        self.uniform.sample(&mut rand::thread_rng()) * 2.0 - 1.0
    }
    pub fn uniform_sample_point2(&self) -> Point2 {
        point2(self.sample_0_1(), self.sample_0_1())
    }

    pub fn get_2d_array(&self, n: S) -> Vec<Point2> {
        let mut out = vec![];
        for _ in 0..n {
            out.push(self.uniform_sample_point2())
        }
        out
    }
}
