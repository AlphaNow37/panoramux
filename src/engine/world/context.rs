
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct TransparentHasher {
    state: u64,
}
impl std::hash::Hasher for TransparentHasher {
    fn finish(&self) -> u64 {
        self.state
    }
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.rotate_left(8) ^ (byte as u64);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.state = i;
    }
    fn write_u128(&mut self, i: u128) {
        self.state = i as u64
    }
}
impl std::hash::BuildHasher for TransparentHasher {
    type Hasher = TransparentHasher;
    fn build_hasher(&self) -> Self::Hasher {
        self.clone()
    }
}

pub struct Context {
    // invariant: the type of the dyn any must be the key
    store: HashMap<TypeId, Box<dyn Any>, TransparentHasher>
}
impl Context {
    pub fn new() -> Self {
        Self {
            store: HashMap::with_hasher(TransparentHasher {state: 0}),
        }
    }
    pub fn set<T>(&mut self, value: T) {
        self.store.insert(TypeId::of::<T>(), Box::new(value));
    }
    pub fn get<T>(&self) -> Option<&T> {
        self.store.get(&TypeId::of::<T>()).and_then(|v| v.downcast_ref())
    }
}
