use std::collections::hash_map::{Entry, HashMap};

use crate::theory::math::traits::{Length, VectorSpace};

use super::container::Container;

pub struct VerticalGraphIterator<'a, Id, G, C> {
    graph: &'a G,
    seens: C,
    stack: Vec<Id>,
}
impl<'a, Id: Clone, G: Graph<Id>, C: Container<Id>> Iterator
    for VerticalGraphIterator<'a, Id, G, C>
{
    type Item = Id;
    fn next(&mut self) -> Option<Self::Item> {
        let top = self.stack.pop()?;
        for new in self.graph.iter_neighboors(top.clone()) {
            if !self.seens.has(&new) {
                self.seens.add(new.clone());
                self.stack.push(new);
            }
        }
        Some(top)
    }
}

pub trait Graph<Id> {
    fn iter_neighboors(&self, node: Id) -> impl Iterator<Item = Id>;

    fn iter_vertical<C: Container<Id>>(&self, start: Id) -> VerticalGraphIterator<'_, Id, Self, C>
    where
        Self: Sized,
    {
        VerticalGraphIterator {
            graph: self,
            seens: C::empty(),
            stack: vec![start],
        }
    }
    fn djikstra(&self, start: Id, end: Id, dist: impl Fn(&Id, &Id) -> f32) -> Option<Vec<Id>>
    where
        Id: std::hash::Hash + Eq + Clone,
    {
        let mut dists = HashMap::new();
        dists.insert(start.clone(), (true, start.clone(), 0.));
        let mut current = start.clone();
        loop {
            if current == end {
                let mut frames = Vec::new();
                while current != start {
                    let new = dists.remove(&current).unwrap().1;
                    frames.push(current);
                    current = new;
                }
                frames.push(start);
                frames.reverse();
                return Some(frames);
            }

            let (done, _, curr_dist) = dists.get_mut(&current).unwrap();
            *done = true;
            let curr_dist = *curr_dist;
            for new in self.iter_neighboors(current.clone()) {
                let entry = dists.entry(new.clone());
                if let Entry::Occupied(mut v) = entry {
                    let (done, last, d) = v.get_mut();
                    if !*done {
                        let new_dist = curr_dist + dist(&current, &new);
                        if *d > new_dist {
                            *d = new_dist;
                            *last = current.clone();
                        }
                    }
                } else {
                    let new_dist = curr_dist + dist(&current, &new);
                    entry.insert_entry((false, new, new_dist));
                }
            }

            current = dists
                .iter()
                .filter(|(_, (done, _, _))| !*done)
                .min_by(|(_, (_, _, d1)), (_, (_, _, d2))| d1.total_cmp(d2))?
                .0
                .clone();
        }
    }
    fn a_star_vspace<V: VectorSpace + Length>(
        &self,
        start: Id,
        end: Id,
        pos: impl Fn(&Id) -> V,
    ) -> Option<Vec<Id>>
    where
        Id: std::hash::Hash + Eq + Clone,
    {
        let end_coord = pos(&end);
        return self.djikstra(start, end, |a, b| {
            let a = pos(a);
            let b = pos(b);
            (end_coord - b).length() + (b - a).length() - (a - end_coord).length()
        });
    }
}

pub trait GraphNodeIterable<Id>: Graph<Id> {
    fn iter_nodes(&self) -> impl Iterator<Item = Id>;
    fn size_hint(&self) -> usize;
}

#[derive(Clone)]
pub struct LinkGraph {
    pub targets: Vec<Vec<usize>>,
}
impl Graph<usize> for LinkGraph {
    fn iter_neighboors(&self, node: usize) -> impl Iterator<Item = usize> {
        return self.targets[node].iter().copied();
    }
}
impl GraphNodeIterable<usize> for LinkGraph {
    fn iter_nodes(&self) -> impl Iterator<Item = usize> {
        0..self.targets.len()
    }
    fn size_hint(&self) -> usize {
        self.targets.len()
    }
}
impl LinkGraph {
    pub fn out_degree(&self, node: usize) -> usize {
        self.targets[node].len()
    }
    pub fn size_exact(&self) -> usize {
        self.targets.len()
    }
}
impl LinkGraph {
    pub fn from_fn(size: usize, f: impl Fn(usize) -> Vec<usize>) -> Self {
        Self {
            targets: (0..size).map(|i| f(i)).collect(),
        }
    }
    pub fn new(size: usize) -> Self {
        Self::from_fn(size, |_| Vec::new())
    }
    pub fn empty() -> Self {
        Self::new(0)
    }
    pub fn push_link(&mut self, start: usize, end: usize) {
        self.targets[start].push(end);
    }
    pub fn push_double_link(&mut self, a: usize, b: usize) {
        self.push_link(a, b);
        self.push_link(b, a);
    }
    pub fn new_node(&mut self) -> usize {
        let i = self.targets.len();
        self.targets.push(Vec::new());
        i
    }
}

#[derive(Clone)]
pub struct GridGraph {
    // Step of each direction, then the total size
    steps_size: Vec<usize>,
    wrapping: bool,
}
impl Graph<usize> for GridGraph {
    fn iter_neighboors(&self, node: usize) -> impl Iterator<Item = usize> {
        (0..self.steps_size.len() - 1)
            .map(move |i| {
                let step = self.steps_size[i];
                let next_size = self.steps_size[i + 1];
                [
                    if node % next_size >= step {
                        Some(node - step)
                    } else if self.wrapping {
                        Some(node + next_size - step)
                    } else {
                        None
                    },
                    if (node % next_size) + step < next_size {
                        Some(node + step)
                    } else if self.wrapping {
                        Some(node + step - next_size)
                    } else {
                        None
                    },
                ]
            })
            .flatten()
            .filter_map(|e| e)
    }
}
impl GraphNodeIterable<usize> for GridGraph {
    fn iter_nodes(&self) -> impl Iterator<Item = usize> {
        0..*self.steps_size.last().unwrap()
    }
    fn size_hint(&self) -> usize {
        *self.steps_size.last().unwrap()
    }
}
impl GridGraph {
    pub fn from_dims(dims: Vec<usize>, wrapping: bool) -> Self {
        let mut steps = vec![0; dims.len() + 1];
        steps[0] = 1;
        for i in 0..dims.len() {
            steps[i + 1] = steps[i] * dims[i]
        }
        Self {
            steps_size: steps,
            wrapping,
        }
    }
    pub fn to_link_graph(&self) -> LinkGraph {
        LinkGraph::from_fn(self.node_count(), |i| self.iter_neighboors(i).collect())
    }
    pub fn node_count(&self) -> usize {
        *self.steps_size.last().unwrap()
    }
    pub fn id_of_coords(&self, coords: &[usize]) -> usize {
        assert_eq!(coords.len(), self.steps_size.len() - 1);
        let mut id = 0;
        for i in 0..coords.len() {
            assert!(
                coords[i] * self.steps_size[i] < self.steps_size[i + 1],
                "Out of bound coords"
            );
            id += coords[i] * self.steps_size[i];
        }
        id
    }
    pub fn coords_of_id_in(&self, id: usize, res: &mut [usize]) {
        assert_eq!(res.len(), self.steps_size.len() - 1);
        for i in 0..res.len() {
            res[i] = (id % self.steps_size[i + 1]) / self.steps_size[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, LinkGraph};

    #[test]
    fn test() {
        const N: usize = 100;
        fn f(i: usize) -> Vec<usize> {
            if i == 0 {
                return Vec::new();
            }
            return ((i + 1)..N).step_by(i).rev().collect();
        }
        let g = LinkGraph::from_fn(N, f);
        dbg!(&g.targets);
        for start in [0, 1, 5] {
            dbg!(start, g.iter_vertical::<Vec<_>>(start).collect::<Vec<_>>());
        }
    }
}
