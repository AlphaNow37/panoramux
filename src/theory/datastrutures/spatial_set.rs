use core::f32;

use crate::theory::math::traits::{Length, VectorSpace};

use super::tree::Tree;

pub trait SpatialSet {
    type Coord;
    type Item;

    fn iter_near<'a>(
        &'a self,
        position: Self::Coord,
        max_dist: f32,
    ) -> impl Iterator<Item = &'a Self::Item>
    where
        Self::Coord: Length + VectorSpace;
}

/// A tree based on a center sphere
///
pub struct DistTree<C, T> {
    subnodes: Vec<DistTreeNode<C, T>>,
    child_radius: f32,
}
struct DistTreeNode<C, T> {
    center: C,
    value: T,
    subtree: DistTree<C, T>,
}
impl<C: VectorSpace + Length, T> DistTree<C, T> {
    pub fn new(rad: f32) -> Self {
        Self {
            subnodes: Vec::new(),
            child_radius: rad,
        }
    }
    pub fn push(&mut self, coord: C, value: T) {
        for node in &mut self.subnodes {
            let delta = coord - node.center;
            let dist = delta.length();
            if dist < self.child_radius {
                node.subtree.push(coord, value);
                return;
            }
        }
        self.subnodes.push(DistTreeNode {
            center: coord,
            value,
            subtree: DistTree::new(self.child_radius / 2.),
        });
    }
}
impl<'b, C, T> Tree for &'b DistTree<C, T> {
    fn iter_childs<'a>(&'a self) -> impl Iterator<Item = Self> {
        self.subnodes.iter().map(|v| &v.subtree)
    }
}
