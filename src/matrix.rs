extern crate nalgebra_glm as glm;
use crate::common::*;

// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct Matrix4 {
//     // m: [[F; 4]; 4],
//     m: glm::Mat4
// }

// impl Default for Matrix4 {
//     fn default() -> Self {
//         Self {
//             m: glm::Mat4::default(),
//         }
//     }
// }
pub type Matrix4 = glm::Mat4;
pub fn matrix4(m: [[F; 4]; 4]) -> Matrix4 {
    glm::mat4(
        m[0][0], m[0][1], m[0][2], m[0][3], m[1][0], m[1][1], m[1][2], m[1][3], m[2][0], m[2][1],
        m[2][2], m[2][3], m[3][0], m[3][1], m[3][2], m[3][3],
    )
}

// matrix inversion formulae from https://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix

// fn invf(i: I, j: I, m: Matrix4) -> F {
//     let o = 2 + (j - i);
//     let ii = i + 4 + o;
//     let jj = j + 4 - o;

//     let e = |a, b| {
//         // let idx = (((jj+b)%4)*4 + ((ii+a)%4)) as S;
//         m[((jj+b)%4) as S][((ii+a)%4) as S]
//     };
//     let inv =
//         e(1,-1)*e(0,0)*e(-1,1)
//         + e(1,1)*e(0,-1)*e(-1,0)
//         + e(-1,-1)*e(1,0)*e(0,1)
//         - e(-1,-1)*e(0,0)*e(1,1)
//         - e(-1,1)*e(0,-1)*e(1,0)
//         - e(1,-1)*e(-1,0)*e(0,1);

//     if o % 2 == 0 {
//         -inv
//     } else {
//         inv
//     }
// }

// impl Matrix4 {
//     pub fn new(m: [[F; 4]; 4]) -> Self { Self{
//            m: glm::mat4(m[0][0], m[0][1], m[0][2], m[0][3],
//                         m[1][0], m[1][1], m[1][2], m[1][3],
//                         m[2][0], m[2][1], m[2][2], m[2][3],
//                         m[3][0], m[3][1], m[3][2], m[3][3],)
//         }
//     }
//     pub fn identity() -> Self {
//         Self {m: glm::identity() }
//     }

//     pub fn transpose(self) -> Matrix4 {
//         Matrix4 {m: self.m.transpose() }
//     }

//     pub fn invert(self) -> Option<Matrix4> {
//         Some(Matrix4 {m: match self.m.try_inverse() {
//             Some(inv) => inv,
//             None => { return None }
//         }})
//     }

//     pub fn at(self, i: S, j: S) -> F {
//         self.m[i*4+j]
//     }
// }

// impl Mul<Matrix4> for Matrix4 {
//     type Output = Matrix4;
//     fn mul(self, rhs: Matrix4) -> Self::Output {
//         Matrix4{m: self.m * rhs.m }
//     }
// }

// // impl IndexMut<S> for Matrix4 {
// //     fn index_mut(&mut self, index: S) -> &mut [F; 4] {
// //         let row = &self.m.row(index);
// //         &mut [row[0], row[1], row[2], row[3]]
// //     }
// // }
