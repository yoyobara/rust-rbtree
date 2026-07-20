use crate::{
    types::{Node, RBTreeSet, RbtreeSetExt},
    utils::{inner_add, inner_size},
};

impl<T: Ord> RbtreeSetExt<T> for RBTreeSet<T> {
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
