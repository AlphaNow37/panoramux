pub struct VerticalTreeIterator<T: Tree, F: Fn(&T) -> bool> {
    todo: Vec<T>,
    walk_condition: F,
}
impl<T: Tree, F: Fn(&T) -> bool> Iterator for VerticalTreeIterator<T, F> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let top_tree = self.todo.pop()?;
        for child in top_tree.iter_childs() {
            if (self.walk_condition)(&child) {
                self.todo.push(child);
            }
        }
        Some(top_tree)
    }
}

pub trait Tree {
    fn iter_childs<'a>(&'a self) -> impl Iterator<Item = Self>;

    fn iter_descendent_if<F: Fn(&Self) -> bool>(self, f: F) -> VerticalTreeIterator<Self, F>
    where
        Self: Sized,
    {
        VerticalTreeIterator {
            todo: vec![self],
            walk_condition: f,
        }
    }
    fn iter_descendent(self) -> VerticalTreeIterator<Self, fn(&Self) -> bool>
    where
        Self: Sized,
    {
        self.iter_descendent_if(|_| true)
    }
    fn find(self, mut f: impl FnMut(&Self) -> bool) -> Option<Self>
    where
        Self: Sized,
    {
        self.iter_descendent().find(move |t| f(t))
    }
}

struct TreeIf<'a, F, T> {
    cond: &'a F,
    tree: T,
}
impl<'a, F: Fn(&T) -> bool, T: Tree> Tree for TreeIf<'a, F, T> {
    fn iter_childs(&self) -> impl Iterator<Item = Self> {
        self.tree
            .iter_childs()
            .filter(|t| (self.cond)(t))
            .map(|t| TreeIf {
                cond: self.cond,
                tree: t,
            })
    }
}
