use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct ArenaIndex<T>(usize, PhantomData<T>);
impl<T> ArenaIndex<T> {
    pub const NONE: Self = Self(usize::MAX, PhantomData);
    pub fn to_int(self) -> usize {
        self.0
    }
    pub fn cast<U>(self) -> ArenaIndex<U> {
        ArenaIndex(self.0, PhantomData)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Arena<T> {
    free_idxs: Vec<usize>,
    pub data: Vec<T>,
    pub is_alive: Vec<bool>,
}
impl<T> Arena<T> {
    pub fn new()->Self {
        Self {
            free_idxs: Vec::new(),
            data: Vec::new(),
            is_alive: Vec::new(),
        }
    }
    pub fn add(&mut self, elt: T) -> ArenaIndex<T> {
        ArenaIndex(match self.free_idxs.pop() {
            None => {
                self.data.push(elt);
                self.is_alive.push(true);
                self.data.len() - 1
            }
            Some(idx) => {
                self.data[idx] = elt;
                debug_assert!(!self.is_alive[idx]);
                self.is_alive[idx] = true;
                idx
            },
        }, PhantomData)
    }
    pub fn remove(&mut self, idx: ArenaIndex<T>) {
        debug_assert!(self.is_alive[idx.0]);
        self.free_idxs.push(idx.0);
        self.is_alive[idx.0] = false;
    }
    pub fn get(&self, idx: ArenaIndex<T>) -> &T {
        debug_assert!(self.is_alive[idx.0]);
        &self.data[idx.0]
    }
    pub fn set(&mut self, idx: ArenaIndex<T>, value: T) {
        debug_assert!(self.is_alive[idx.0]);
        self.data[idx.0] = value;
    }
    pub fn get_mut(&mut self, idx: ArenaIndex<T>) -> &mut T {
        debug_assert!(self.is_alive[idx.0]);
        &mut self.data[idx.0]
    }
}
