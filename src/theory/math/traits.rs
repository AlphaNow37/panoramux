use crate::theory::utils::macros::make_trait_alias;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Zero {
    const ZERO: Self;
}
impl Zero for f32 {
    const ZERO: Self = 0.;
}
impl Zero for usize {
    const ZERO: Self = 0;
}
impl<T: Zero, const N: usize> Zero for [T; N] {
    const ZERO: Self = [T::ZERO; N];
}

make_trait_alias!(
    VectorSpace = [
        Sized
        + Add<Output=Self>
        + AddAssign
        + Sub<Output=Self>
        + SubAssign
        + Mul<f32, Output=Self>
        + MulAssign<f32>
        + Div<f32, Output=Self>
        + DivAssign<f32>
        + Copy
        + Zero
    ] {
        fn mid(self, other: Self) -> Self {
            (self + other) * 0.5
        }
    }
);

pub trait Length: VectorSpace {
    fn length_squared(self) -> f32;
    fn length(self) -> f32 {
        return self.length_squared().sqrt();
    }
    fn with_length(self, length: f32) -> Self {
        self * length / self.length()
    }
    fn with_length_squared(self, length: f32) -> Self {
        self * (length / self.length_squared()).sqrt()
    }
    fn with_length_or_zero_squared(self, length: f32) -> Self {
        let l = self.length_squared();
        if l == 0. {
            Self::ZERO
        } else {
            self * (length / l).sqrt()
        }
    }
    fn with_length_or_zero(self, length: f32) -> Self {
        let len = self.length();
        if len == 0. {
            Self::ZERO
        } else {
            self * length / len
        }
    }
    fn normalize(self) -> Self {
        self / self.length()
    }
    fn normalize_or_zero(self) -> Self {
        let len = self.length();
        if len == 0. { Self::ZERO } else { self / len }
    }
    fn is_normalized(self) -> bool {
        let l = self.length_squared();
        0.99 < l && l < 1.01
    }
    fn is_approx_zero(self) -> bool {
        self.length_squared() < F32_EPSILON
    }
}

// pub trait VectorSpace: Sized+Add<Output=Self> + Sub<Output=Self> + Mul<f32, Output=Self> + Div<f32, Output=Self> + Copy + Zero {
//     fn mid(self, other: Self) -> Self {
//         (self + other) * 0.5
//     }
// }
// impl<T: Sized+Add<Output=Self> + Sub<Output=Self> + Mul<f32, Output=Self> + Div<f32, Output=Self> + Copy + Zero> VectorSpace for T {}

macro_rules! impl_vector_space_simd {
    (
        $t: ident ($n: literal)
    ) => {
        use crate::theory::math::traits::Zero;
        use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
        use std::simd::ToBytes;

        impl $t {
            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.simd_clamp(min.0, max.0))
            }
        }
        impl Zero for $t {
            const ZERO: Self = Self(Simd::from_array([0.; $n]));
        }
        impl Add for $t {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        impl AddAssign for $t {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }
        impl Sub for $t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        impl SubAssign for $t {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }
        impl Mul<f32> for $t {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                Self(self.0 * Simd::splat(rhs))
            }
        }
        impl MulAssign<f32> for $t {
            fn mul_assign(&mut self, rhs: f32) {
                self.0 *= Simd::splat(rhs)
            }
        }
        impl Div<f32> for $t {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                Self(self.0 / Simd::splat(rhs))
            }
        }
        impl DivAssign<f32> for $t {
            fn div_assign(&mut self, rhs: f32) {
                self.0 /= Simd::splat(rhs)
            }
        }
        impl Neg for $t {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }
        impl Default for $t {
            fn default() -> Self {
                Self::ZERO
            }
        }
        impl std::hash::Hash for $t {
            fn hash<T: std::hash::Hasher>(&self, hasher: &mut T) {
                self.0.to_ne_bytes().hash(hasher)
            }
        }
    };
}
pub(crate) use impl_vector_space_simd;
use crate::theory::utils::number_ext::F32_EPSILON;

