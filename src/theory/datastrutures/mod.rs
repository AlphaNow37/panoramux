use std::cell::Cell;
use std::rc::Rc;

pub mod container;
pub mod graph;
pub mod sampler_linker;
pub mod spatial_set;
pub mod tree;
pub mod arena;
pub mod priority_queue;
pub mod union_find;

pub type RCell<T> = Rc<Cell<T>>;
pub fn rcell<T>(val: T) -> RCell<T> {
    Rc::new(Cell::new(val))
}
