
/// Can hash floats. Is just supposed to be a utility to find variables with the same value
pub trait GeneralHash {
    fn gen_hash(&self) -> u32;
}
macro_rules! impl_hash {
    (
        $self: ident,
        $(
            $ty: ty:
            $(<$($gen: ident), * $(,)?>)?
            $([$($gen2: tt)*])?
            { $expr: expr }
        );* $(;)?
    ) => {
        $(
            impl <$($($gen: GeneralHash, )*)? $($($gen2)*)?> GeneralHash for $ty {
                fn gen_hash(&$self) -> u32 {
                    $expr
                }
            }
        )*
    };
}

fn gen_hash_stdhash(v: &impl std::hash::Hash) -> u32 {
    struct Hasher(u32);
    impl std::hash::Hasher for Hasher {
        fn finish(&self) -> u64 {
            unimplemented!()
        }
        fn write(&mut self, bytes: &[u8]) {
            self.0 = (self.0 << 1) ^ bytes.gen_hash();
        }
        fn write_u32(&mut self, i: u32) {
            self.0 = (self.0 << 1) ^ i
        }
        fn write_u64(&mut self, i: u64) {
            self.0 = ((self.0 << 1) ^ (i as u32) << 1) ^ ((i >> 32) as u32)
        }
    }
    let mut h = Hasher(0);
    v.hash(&mut h);
    h.0
}

use crate::engine::components::camera::Camera;
use crate::engine::components::color::Color;
use crate::theory::math::{Angle, Dir, Mat4, Plane, Polynomial, Transform, Vec2, Vec3, Vec4};
use std::any::TypeId;

impl_hash!(
    self,
    f32: {self.to_bits().into()};
    usize: {*self as u32};
    u8: {*self as u32};
    u32: {*self};
    (A, B): <A, B> {(self.0.gen_hash() << 1) ^ self.1.gen_hash()};
    (A, B, C): <A, B, C> {((self.0.gen_hash()<<1) ^ self.1.gen_hash() << 1) ^ self.2.gen_hash()};
    (A, B, C, D): <A, B, C, D> {(((self.0.gen_hash()<<1) ^ self.1.gen_hash()<<1) ^ self.2.gen_hash()<<1) ^ self.3.gen_hash()};
    &[T]: <T> {self.iter().map(|e| e.gen_hash()).fold(0, |a, b| (a<<1)^b)};
    [T; N]: <T> [const N: usize] {self.iter().map(|e| e.gen_hash()).fold(0, |a, b| (a<<1)^b)};
    Angle: {self.rad().rem_euclid(std::f32::consts::TAU).gen_hash()};
    Vec2: {self.0.as_array().gen_hash()};
    Vec3: {(&self.0.as_array()[..3]).gen_hash()};
    Vec4: {self.0.as_array().gen_hash()};
    Mat4: {self.0.as_array().gen_hash()};
    Transform: {self.0.as_array().gen_hash()};
    Polynomial<T, N, M>: <T> [const N: usize, const M: usize] {self.0.gen_hash()};
    Dir: {(**self).gen_hash()};
    Plane: {self.normal().gen_hash()};
    Color: {self.to_array().gen_hash()};
    Camera: {(self.fov, self.pos).gen_hash()};
    TypeId: {gen_hash_stdhash(self)};
);
