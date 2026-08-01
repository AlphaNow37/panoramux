pub trait UsizeExt {
    fn add_rem(self, other: isize, rem: usize) -> usize;
}
impl UsizeExt for usize {
    fn add_rem(self, other: isize, rem: usize) -> usize {
        (self as isize + other).rem_euclid(rem as isize) as usize
    }
}

pub const F32_EPSILON: f32 = 1.0e-5;
