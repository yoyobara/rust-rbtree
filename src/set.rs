use crate::utils::{Node, inner_add, inner_size};

pub struct RBTreeSet<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> RBTreeSet<T> {
    fn add(&mut self, value: T) {
        let new_node = Node {
            value,
            left: None,
            right: None,
        };

        inner_add(&mut self.root, new_node);
    }

    fn contains(&self, value: T) -> bool {
        todo!()
    }

    fn remove(&self, value: T) -> bool {
        todo!()
    }

    fn size(&self) -> usize {
        inner_size(&self.root)
    }
}
