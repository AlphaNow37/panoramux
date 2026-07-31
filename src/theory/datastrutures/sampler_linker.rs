use rand::{Rng, RngExt};

use crate::theory::math::traits::{Length, VectorSpace};
use crate::theory::datastrutures::graph::GridGraph;

/// pairs:
///    C
///    |
///    |
/// A--+--B
///    |
///    |
///    D

pub struct DimensionParam<T> {
    pub a: T,
    pub b: T,
    pub point_amount: usize,
    // Average variation from the center in a [0; +oo] scale. 0=base place, 1=until neighboor
    pub mean_variation: f32,
}
pub struct SampleLinkPointParam<const NDIM: usize, T> {
    pub dims: [DimensionParam<T>; NDIM],
}

impl<const NDIM: usize, T: VectorSpace + Length> SampleLinkPointParam<NDIM, T> {
    pub fn eval(&self, rng: &mut impl Rng) -> (GridGraph, Vec<T>) {
        let graph = GridGraph::from_dims(self.dims.iter().map(|d| d.point_amount).collect(), false);

        let count = graph.node_count();
        let mut points = vec![T::ZERO; count];
        let mut coords = vec![0; NDIM];
        for i in 0..count {
            graph.coords_of_id_in(i, &mut coords);
            for k in 0..NDIM {
                let dim = &self.dims[k];
                let dt = 1. / ((dim.point_amount - 1) as f32);
                let part = (coords[k] as f32 + rng.random_range(-0.5..0.5) * dim.mean_variation) * dt;
                points[i] += dim.a * part + dim.b * (1. - part);
            }
        }
        (graph, points)
    }
}
