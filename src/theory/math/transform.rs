use crate::theory::math::traits::{Length, impl_vector_space_simd};
use crate::theory::math::{Angle, Dir, Mat4, Vec3};
use std::fmt::{Debug, Display, Formatter};
use std::simd::num::SimdFloat;
use std::simd::prelude::SimdPartialOrd;
use std::simd::{Simd, StdFloat, simd_swizzle};

pub const fn scale(x: f32, y: f32, z: f32) -> Transform {
    Transform::from_scalef(x, y, z)
}
pub const fn trans(x: f32, y: f32, z: f32) -> Transform {
    Transform::from_transf(x, y, z)
}
pub fn rotate_x(a: Angle) -> Transform {
    Transform::from_rotate_x(a)
}
pub fn rotate_y(a: Angle) -> Transform {
    Transform::from_rotate_y(a)
}
pub fn rotate_z(a: Angle) -> Transform {
    Transform::from_rotate_z(a)
}
pub fn rotate_around(axis: impl TryInto<Dir>, a: Angle) -> Transform {
    Transform::from_rotate_around(axis, a)
}

/// Transform
/// array layout:
/// - 3 columns vectors for the mat3
/// - 1 column vector for the translation
/// Notice that the indexes at 3, 7, 11, 15 have to be 0
#[derive(Copy, Clone, PartialEq)]
pub struct Transform(pub Simd<f32, 16>);
impl Transform {
    pub const X: Self = Self::from_scalef(1., 0., 0.);
    pub const Y: Self = Self::from_scalef(0., 1., 0.);
    pub const Z: Self = Self::from_scalef(0., 0., 1.);
    pub const ID: Self = Self::from_scalef(1., 1., 1.);
    pub const EPSILON: Self = Self::from_array([f32::EPSILON; 12]);

    pub const fn from_array(arr: [f32; 12]) -> Self {
        Self(Simd::from_array([
            arr[0], arr[1], arr[2], 0., arr[3], arr[4], arr[5], 0., arr[6], arr[7], arr[8], 0.,
            arr[9], arr[10], arr[11], 0.,
        ]))
    }
    pub const fn from_cols(x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self::from_array([
            x.x(),
            x.y(),
            x.z(),
            y.x(),
            y.y(),
            y.z(),
            z.x(),
            z.y(),
            z.z(),
            0.,
            0.,
            0.,
        ])
    }
    pub const fn from_rows(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self::from_array([
            a.x(),
            b.x(),
            c.x(),
            a.y(),
            b.y(),
            c.y(),
            a.z(),
            b.z(),
            c.z(),
            0.,
            0.,
            0.,
        ])
    }
    pub const fn from_scalef(x: f32, y: f32, z: f32) -> Self {
        Self(Simd::from_array([
            x, 0., 0., 0., 0., y, 0., 0., 0., 0., z, 0., 0., 0., 0., 0.,
        ]))
    }
    pub fn from_scalev(vec: Vec3) -> Self {
        Self::from_scalef(vec.x(), vec.y(), vec.z())
    }
    pub const fn from_transf(x: f32, y: f32, z: f32) -> Self {
        Self(Simd::from_array([
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., x, y, z, 0.,
        ]))
    }
    pub const fn from_transv(vec: Vec3) -> Self {
        Self::from_transf(vec.x(), vec.y(), vec.z())
    }
    /// View from the end of the vector, looking opposite way
    pub fn from_rotate_x(angle: Angle) -> Self {
        Self::from_array([
            1.,
            0.,
            0.,
            0.,
            angle.cos(),
            angle.sin(),
            0.,
            -angle.sin(),
            angle.cos(),
            0.,
            0.,
            0.,
        ])
    }
    pub fn from_rotate_y(angle: Angle) -> Self {
        Self::from_array([
            angle.cos(),
            0.,
            -angle.sin(),
            0.,
            1.,
            0.,
            angle.sin(),
            0.,
            angle.cos(),
            0.,
            0.,
            0.,
        ])
    }
    pub fn from_rotate_z(angle: Angle) -> Self {
        Self::from_array([
            angle.cos(),
            angle.sin(),
            0.,
            -angle.sin(),
            angle.cos(),
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
        ])
    }
    pub fn from_rotate_around(axis: impl TryInto<Dir>, angle: Angle) -> Self {
        Self::ID.rotate_around(axis, angle)
    }
    pub fn from_z_looking_at(axis: impl TryInto<Dir>) -> Self {
        let Result::<Dir, _>::Ok(dir) = axis.try_into() else {
            return Self::ZERO;
        };
        let mut normal = dir.cross(Vec3::X);
        if normal.is_approx_zero() {
            normal = dir.cross(Vec3::Y);
        }
        let last_normal = normal.cross(*dir);
        Self::from_cols(
            last_normal.normalize_or_zero(),
            normal.normalize_or_zero(),
            *dir,
        )
    }

    pub fn trans(self) -> Vec3 {
        Vec3(simd_swizzle!(self.0, [12, 13, 14, 15]))
    }
    pub fn x(self) -> Vec3 {
        Vec3(simd_swizzle!(self.0, [0, 1, 2, 3]))
    }
    pub fn y(self) -> Vec3 {
        Vec3(simd_swizzle!(self.0, [4, 5, 6, 7]))
    }
    pub fn z(self) -> Vec3 {
        Vec3(simd_swizzle!(self.0, [8, 9, 10, 11]))
    }
    pub fn scale_squared(self) -> Vec3 {
        fn square(sim: Simd<f32, 4>) -> Simd<f32, 4> {
            sim * sim
        }
        Vec3(
            square(simd_swizzle!(self.0, [0, 4, 8, 3]))
                + square(simd_swizzle!(self.0, [1, 5, 9, 3]))
                + square(simd_swizzle!(self.0, [2, 6, 10, 3])),
        )
    }
    pub fn scale(self) -> Vec3 {
        Vec3(self.scale_squared().0.sqrt())
    }

    pub fn approx_eq(self, other: Self) -> bool {
        (self - other).0.abs().simd_lt(Simd::splat(0.0001)).all()
    }
    pub fn to_array(self) -> [f32; 12] {
        let a = self.0.to_array();
        [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14].map(|i| a[i])
    }
    pub fn to_mat4(mut self) -> Mat4 {
        self.0[15] = 1.;
        debug_assert_eq!(self.0[3], 0.);
        debug_assert_eq!(self.0[7], 0.);
        debug_assert_eq!(self.0[11], 0.);
        Mat4(self.0)
    }

    pub fn tr_point(self, pt: Vec3) -> Vec3 {
        self.tr_vec(pt) + self.trans()
    }
    pub fn tr_vec(self, vec: Vec3) -> Vec3 {
        self.x() * vec.x() + self.y() * vec.y() + self.z() * vec.z()
    }
    pub fn tr_tr(self, tr: Self) -> Self {
        // hopefully this (long!) formula is right ! (and faster than the naive method)
        Self(
            simd_swizzle!(self.0, [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3,])
                * simd_swizzle!(tr.0, [0, 0, 0, 0, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 12,])
                + simd_swizzle!(self.0, [4, 5, 6, 3, 4, 5, 6, 3, 4, 5, 6, 3, 4, 5, 6, 3,])
                    * simd_swizzle!(tr.0, [1, 1, 1, 1, 5, 5, 5, 5, 9, 9, 9, 9, 13, 13, 13, 13,])
                + simd_swizzle!(
                    self.0,
                    [8, 9, 10, 3, 8, 9, 10, 3, 8, 9, 10, 3, 8, 9, 10, 3,]
                ) * simd_swizzle!(tr.0, [
                    2, 2, 2, 2, 6, 6, 6, 6, 10, 10, 10, 10, 14, 14, 14, 14,
                ])
                + simd_swizzle!(self.0, [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 12, 13, 14, 3,]),
        )
    }

    pub fn scaled(self, mut by: Vec3) -> Self {
        by.0[3] = 1.;
        Self(self.0 * simd_swizzle!(by.0, [0, 0, 0, 3, 1, 1, 1, 3, 2, 2, 2, 3, 3, 3, 3, 3,]))
    }
    pub fn translate(self, by: Vec3) -> Self {
        // should be more performant than self*trans(by)
        let v = self.tr_vec(by);
        self + Self(simd_swizzle!(v.0, [
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 1, 2, 3,
        ]))
    }
    pub fn rotate_around(self, axis: impl TryInto<Dir>, angle: Angle) -> Self {
        let Ok(dir) = axis.try_into() else {
            return self;
        };
        Self::from_cols(
            self.x().rotate_around(dir, angle),
            self.y().rotate_around(dir, angle),
            self.z().rotate_around(dir, angle),
        )
    }
    pub fn inverse(self) -> Self {
        let t0 = self.y().cross(self.z());
        let t1 = self.z().cross(self.x());
        let t2 = self.x().cross(self.y());
        let det = self.z().dot(t2);
        debug_assert_ne!(det, 0., "Non reversible matrix !");
        let mat_inverse = Self::from_rows(t0, t1, t2) / det;
        mat_inverse.translate(-self.trans())
    }
    pub fn distance_with(self, pt: Vec3) -> f32 {
        let poss = self.inverse().tr_point(pt);
        poss.length()
    }
    pub fn with_rotation(self, other: Self) -> Self {
        Self(simd_swizzle!(self.0, other.0, [
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 12, 13, 14, 15,
        ]))
    }
    pub fn with_trans(self, other: Self) -> Self {
        other.with_rotation(self)
    }
}
impl_vector_space_simd!(Transform(16));
impl Debug for Transform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transform(tr={:?}, x={:?}, y={:?}, z={:?})",
            self.trans(),
            self.x(),
            self.y(),
            self.z()
        )
    }
}
impl Display for Transform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
impl Mul for Transform {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.tr_tr(rhs)
    }
}
impl MulAssign for Transform {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Add<Vec3> for Transform {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        self.translate(rhs)
    }
}
impl AddAssign<Vec3> for Transform {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = self.translate(rhs)
    }
}
impl Sub<Vec3> for Transform {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self.translate(-rhs)
    }
}
impl SubAssign<Vec3> for Transform {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = self.translate(-rhs)
    }
}

impl Mul<Vec3> for Transform {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.scaled(rhs)
    }
}
impl MulAssign<Vec3> for Transform {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = self.scaled(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_a() {
        assert_eq!(Transform::ID * Transform::ID, Transform::ID);
        assert_eq!(scale(1., 2., 3.) * scale(2., 3., 4.), scale(2., 6., 12.));
        assert_eq!(trans(1., 2., 3.) * trans(2., 3., 4.), trans(3., 5., 7.));
        assert_eq!(
            trans(1., 2., 3.) * scale(2., 3., 4.),
            scale(2., 3., 4.) * trans(0.5, 2. / 3., 3. / 4.)
        );
        let m =
            trans(0., 1., 5.) * scale(2., 5., -6.) * rotate_around(Vec3::ONE, Angle::from_deg(45.));
        assert!(m.inverse().inverse().approx_eq(m));

        let m = trans(1., 0., 0.) * scale(2., 2., 2.);
        let m2 = trans(-0.5, 0., 0.) * scale(0.5, 0.5, 0.5);
        assert!(m.inverse().approx_eq(m2));
    }
}
