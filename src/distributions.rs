use std::f32::consts::PI;

use crate::{
    common::*,
    vector::{point2, vec3, Point2, Vec3},
};

pub struct Distribution1D {
    pub func: Vec<F>,
    pub cdf: Vec<F>,
    pub func_int: F,
}

impl Distribution1D {
    pub fn new(f: &[F], n: S) -> Self {
        let func = Vec::<F>::from(f);
        let mut cdf = vec![0.0];
        for i in 1..n + 1 {
            cdf.push(cdf[cdf.len() - 1] + func[i - 1] / n as F);
        }
        let func_int = cdf[n];
        if func_int == 0.0 {
            for i in 1..n + 1 {
                cdf[i] = i as F / n as F;
            }
        } else {
            for i in 1..n + 1 {
                cdf[i] /= func_int;
            }
        }
        Self {
            func,
            cdf,
            func_int,
        }
    }
    pub fn count(&self) -> S {
        self.func.len()
    }

    pub fn sample_discrete(&self, u: F) -> Option<(F, F)> {
        self.cdf
            .iter()
            .enumerate()
            .find(|(_, x)| **x <= u)
            .map(|(offset, _)| {
                (
                    self.func[offset] / (self.func_int / self.count() as F),
                    (u - self.cdf[offset]) / (self.cdf[offset + 1] - self.cdf[offset]),
                )
            })
    }
    pub fn discrete_pdf(&self, index: S) -> F {
        self.func[index] / (self.func_int * self.count() as F)
    }

    pub fn sample_continuous(&self, u: F) -> Option<(F, F, S)> {
        if let Some((offset, _)) = self.cdf.iter().enumerate().find(|(_, x)| **x <= u) {
            let mut du = u - self.cdf[offset];
            if self.cdf[offset + 1] - self.cdf[offset] > 0.0 {
                du /= self.cdf[offset + 1] - self.cdf[offset];
            }
            let pdf = self.func[offset] / self.func_int;
            Some(((offset as F + du) / self.count() as F, pdf, offset))
        } else {
            None
        }
    }

    pub fn uniform_sample_hemisphere(u: &Point2) -> Vec3 {
        let z = u.x;
        let r = F::sqrt(F::max(0.0, 1.0 - z * z));
        let phi = 2.0 * PI * u.y;
        vec3(r * phi.cos(), r * phi.sin(), z)
    }
    pub fn uniform_hemisphere_pdf() -> F {
        1.0 / (2.0 * PI)
    }

    pub fn uniform_sample_sphere(u: &Point2) -> Vec3 {
        let z = 1.0 - 2.0 * u.x;
        let r = F::sqrt(F::max(0.0, 1.0 - z * z));
        let phi = 2.0 * PI * u.y;
        vec3(r * phi.cos(), r * phi.sin(), z)
    }
    pub fn uniform_sphere_pdf() -> F {
        1.0 / (4.0 * PI)
    }

    pub fn uniform_sample_disk(u: &Point2) -> Point2 {
        let r = u.x.sqrt();
        let theta = 2.0 * PI * u.y;
        point2(r * theta.cos(), r * theta.sin())
    }

    pub fn uniform_sample_triangle(u: &Point2) -> Point2 {
        let su0 = u.x.sqrt();
        point2(1.0 - su0, u.y * su0)
    }

    pub fn concentric_sample_disk(u: &Point2) -> Point2 {
        let u_offset = 2.0 * u - point2(1.0, 1.0);
        if u_offset.x == 0.0 && u_offset.y == 0.0 {
            return point2(0.0, 0.0);
        }
        let (theta, r) = match u_offset.x.abs() > u_offset.y.abs() {
            true => ((PI / 4.0) * (u_offset.y / u_offset.x), u_offset.x),
            false => ((PI / 4.0) * (u_offset.x / u_offset.y), u_offset.y),
        };
        r * point2(theta.cos(), theta.sin())
    }

    pub fn cosine_sample_hemisphere(u: &Point2) -> Vec3 {
        let d = Distribution1D::concentric_sample_disk(u);
        let z = F::sqrt(F::max(0.0, 1.0 - d.x * d.x - d.y * d.y));
        vec3(d.x, d.y, z)
    }
    pub fn cosine_hemisphere_pdf(cos_theta: F) -> F {
        cos_theta / PI
    }
}

pub struct Distribution2D {
    p_conditional_v: Vec<Distribution1D>,
    p_marginal: Distribution1D,
}

impl Distribution2D {
    pub fn new(func: &[&[F]]) -> Self {
        let mut p_conditional_v = vec![];
        for v in 0..func[0].len() {
            p_conditional_v.push(Distribution1D::new(func[..][v], func.len()));
        }
        let mut marginal_func = vec![];
        for cond in p_conditional_v.iter() {
            marginal_func.push(cond.func_int);
        }
        let p_marginal = Distribution1D::new(marginal_func.as_slice(), marginal_func.len());
        Self {
            p_conditional_v,
            p_marginal,
        }
    }

    pub fn sample_continuous(&self, u: &Point2) -> Option<(Point2, F)> {
        if let Some((d1, pdf_1, v)) = self.p_marginal.sample_continuous(u.y) {
            if let Some((d0, pdf_0, _)) = self.p_conditional_v[v].sample_continuous(u.x) {
                Some((point2(d0, d1), pdf_0 * pdf_1))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn pdf(&self, p: Point2) -> F {
        let iu = I::clamp(
            (p[0] * self.p_conditional_v[0].count() as F) as I,
            0,
            self.p_conditional_v[0].count() as I - 1,
        ) as S;
        let iv = I::clamp(
            (p[1] * self.p_marginal.count() as F) as I,
            0,
            self.p_marginal.count() as I - 1,
        ) as S;
        self.p_conditional_v[iv].func[iu] / self.p_marginal.func_int
    }
}
