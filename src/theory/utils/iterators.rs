use std::ops::Range;

pub struct Count(usize);
impl Count {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn curr(&self) -> usize {
        self.0
    }
    pub fn next(&mut self) -> usize {
        self.0 += 1;
        self.0 - 1
    }
    pub fn after(&mut self, nth: usize) -> usize {
        self.0 += nth;
        self.0
    }
    pub fn range_of(&mut self, width: usize) -> Range<usize> {
        self.0..self.after(width)
    }
}
