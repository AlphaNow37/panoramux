use crate::theory::math::traits::{Length, impl_vector_space_simd};
use crate::theory::math::{Angle, Dir, Vec4};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;
use std::simd::num::SimdFloat;
use std::simd::{Simd, simd_swizzle};

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

/// Vec3, the fourth value have to be 0
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Vec3(pub Simd<f32, 4>);
impl Vec3 {
    pub const X: Self = Self::new(1., 0., 0.);
    pub const Y: Self = Self::new(0., 1., 0.);
    pub const Z: Self = Self::new(0., 0., 1.);
    pub const ONE: Self = Self::new(1., 1., 1.);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Simd::from_array([x, y, z, 0.]))
    }
    pub const fn from_array(arr: [f32; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
    pub fn translate(self, other: Self) -> Self {
        self + other
    }
    pub fn map_comp(self, mut f: impl FnMut(f32) -> f32) -> Self {
        Self::new(f(self.x()), f(self.y()), f(self.z()))
    }
    pub fn dir(self) -> Option<Dir> {
        Dir::try_from(self).ok()
    }
    pub const fn x(self) -> f32 {
        self.0.as_array()[0]
    }
    pub const fn y(self) -> f32 {
        self.0.as_array()[1]
    }
    pub const fn z(self) -> f32 {
        self.0.as_array()[2]
    }
    pub fn dot(self, other: Self) -> f32 {
        (self.0 * other.0).reduce_sum()
    }
    pub fn cross(self, other: Self) -> Self {
        // See wikipedia cross formula
        Self(
            simd_swizzle!(self.0, [1, 2, 0, 3]) * simd_swizzle!(other.0, [2, 0, 1, 3])
                - simd_swizzle!(self.0, [2, 0, 1, 3]) * simd_swizzle!(other.0, [1, 2, 0, 3]),
        )
    }
    pub fn square_comps(self) -> Self {
        Self(self.0 * self.0)
    }
    pub fn rotate_around(self, axis: impl TryInto<Dir>, angle: Angle) -> Self {
        let Ok(dir) = axis.try_into() else {
            return self;
        };
        let z = dir.project(self);
        let x = self - z;
        let y = dir.cross(x);
        z + x * angle.cos() + y * angle.sin()
    }
    pub fn to_array(self) -> [f32; 3] {
        [self.x(), self.y(), self.z()]
    }
    pub fn to_vec4(self, w: f32) -> Vec4 {
        Vec4::new(self.x(), self.y(), self.z(), w)
    }
}
impl_vector_space_simd!(Vec3(4));
impl Length for Vec3 {
    fn length_squared(self) -> f32 {
        self.dot(self)
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x(), self.y(), self.z())
    }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self::from_array(value)
    }
}
impl From<Vec3> for [f32; 3] {
    fn from(value: Vec3) -> Self {
        value.to_array()
    }
}
impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0.as_array()[index]
    }
}