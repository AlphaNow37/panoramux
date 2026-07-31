
#[derive(Default, Debug, Clone)]
pub struct Graph {
    neighbors: Vec<Vec<usize>>,
}

impl Graph {
    pub fn add_link(&mut self, start: usize, end: usize) {
        if start >= self.neighbors.len() {
            self.neighbors.extend((0..(start - self.neighbors.len() + 1)).map(|_| Vec::new()))
        }
        self.neighbors[start].push(end)
    }
    pub fn add_bilink(&mut self, a: usize, b: usize) {
        self.add_link(a, b);
        self.add_link(b, a);
    }
    pub fn neighbors_of(&self, v: usize) -> &[usize] {
        &self.neighbors[v]
    }
}
