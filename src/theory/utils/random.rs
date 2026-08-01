use rand::{Rng, RngExt};
use std::ops::Range;

pub struct WeightedDistr<T> {
    offset_value: Vec<(f32, T)>,
    total_weight: f32, // > 0
}
impl<T> WeightedDistr<T> {
    pub fn new(values_weights: impl IntoIterator<Item = (T, f32)>) -> Self {
        let mut offset_value = Vec::new();
        let mut curr_sum = 0.;
        for (val, w) in values_weights.into_iter() {
            debug_assert!(w >= 0.);
            offset_value.push((curr_sum, val));
            curr_sum += w;
        }
        debug_assert!(curr_sum > 0.);
        Self {
            offset_value,
            total_weight: curr_sum,
        }
    }
    pub fn new_rng(
        value_range: impl IntoIterator<Item = (T, Range<f32>)>,
        rng: &mut impl Rng,
        w_fn: fn(f32)->f32,
    ) -> Self {
        Self::new(
            value_range
                .into_iter()
                .map(|(val, range)| (val, w_fn(rng.random_range(range)))),
        )
    }
}
impl<T: Clone> rand::distr::Distribution<T> for WeightedDistr<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        let cursor = rng.random_range(0.0..self.total_weight);
        for (off, val) in &self.offset_value {
            if *off >= cursor {
                return val.clone();
            }
        }
        unreachable!()
    }
}
