use std::collections::HashSet;
use std::hash::Hash;

pub trait Container<T> {
    fn empty() -> Self;
    fn has(&self, key: &T) -> bool;
    fn add(&mut self, key: T);
}

impl<T: PartialEq> Container<T> for Vec<T> {
    fn empty() -> Self {
        Vec::new()
    }
    fn has(&self, key: &T) -> bool {
        self.contains(key)
    }
    fn add(&mut self, key: T) {
        self.push(key);
    }
}
impl<T: Hash + Eq> Container<T> for HashSet<T> {
    fn empty() -> Self {
        HashSet::new()
    }
    fn has(&self, key: &T) -> bool {
        self.contains(key)
    }
    fn add(&mut self, key: T) {
        self.insert(key);
    }
}
