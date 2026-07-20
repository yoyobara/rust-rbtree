use crate::utils::{Node, inner_add, inner_size};

pub struct RBTreeSet<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> RBTreeSet<T> {
    pub fn new() -> Self {
        RBTreeSet { root: None }
    }

    pub fn add(&mut self, value: T) {
        let new_node = Node {
            value,
            left: None,
            right: None,
        };

        inner_add(&mut self.root, new_node);
    }

    pub fn contains(&self, value: T) -> bool {
        todo!()
    }

    pub fn remove(&self, value: T) -> bool {
        todo!()
    }

    pub fn size(&self) -> usize {
        inner_size(&self.root)
    }
}
