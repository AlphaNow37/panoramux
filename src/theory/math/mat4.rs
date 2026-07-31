use crate::theory::math::traits::impl_vector_space_simd;
use crate::theory::math::Angle;
use bytemuck::{Pod, Zeroable};
use std::array::from_fn;
use std::fmt::{Debug, Formatter};
use std::simd::num::SimdFloat;
use std::simd::prelude::SimdPartialOrd;
use std::simd::{Simd, simd_swizzle};

#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct Mat4(pub Simd<f32, 16>);
impl Mat4 {
    pub const fn from_array(arr: [f32; 16]) -> Self {
        Self(Simd::from_array(arr))
    }
    pub const fn from_diag(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Simd::from_array([
            x, 0., 0., 0., 0., y, 0., 0., 0., 0., z, 0., 0., 0., 0., w,
        ]))
    }

    /// See glam::Mat4:perspective_infinite_lh
    pub fn new_perspective_infinite_lh(fovy: Angle, aspect_ratio: f32, z_near: f32) -> Self {
        let h = (fovy * 0.5).cotan();
        let w = h / aspect_ratio;
        Self::from_array([
            w, 0., 0., 0., 0., h, 0., 0., 0., 0., 1., 1., 0., 0., -z_near, 0.,
        ])
    }

    pub fn approx_eq(self, other: Self) -> bool {
        (self - other).0.abs().simd_lt(Simd::splat(0.0001)).all()
    }
    pub fn to_array(self) -> [f32; 16] {
        self.0.to_array()
    }

    pub fn matmul(self, mat: Self) -> Self {
        Self(
            simd_swizzle!(self.0, [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3,])
                * simd_swizzle!(mat.0, [0, 0, 0, 0, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 12,])
                + simd_swizzle!(self.0, [4, 5, 6, 7, 4, 5, 6, 7, 4, 5, 6, 7, 4, 5, 6, 7,])
                    * simd_swizzle!(mat.0, [1, 1, 1, 1, 5, 5, 5, 5, 9, 9, 9, 9, 13, 13, 13, 13,])
                + simd_swizzle!(self.0, [
                    8, 9, 10, 11, 8, 9, 10, 11, 8, 9, 10, 11, 8, 9, 10, 11,
                ]) * simd_swizzle!(mat.0, [
                    2, 2, 2, 2, 6, 6, 6, 6, 10, 10, 10, 10, 14, 14, 14, 14,
                ])
                + simd_swizzle!(self.0, [
                    12, 13, 14, 15, 12, 13, 14, 15, 12, 13, 14, 15, 12, 13, 14, 15,
                ]) * simd_swizzle!(mat.0, [
                    3, 3, 3, 3, 7, 7, 7, 7, 11, 11, 11, 11, 15, 15, 15, 15,
                ]),
        )
    }
}
impl_vector_space_simd!(Mat4(16));
impl Debug for Mat4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut blocs = self.0.as_array().chunks(4);
        let [a, b, c, d] = from_fn(|_| blocs.next().unwrap());
        write!(f, "Mat4({:?}, {:?}, {:?}, {:?})", a, b, c, d)
    }
}
impl Mul for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.matmul(rhs)
    }
}
