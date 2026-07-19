use std::cmp::Ordering;

use crate::types::Node;

pub fn inner_add<T: Ord>(root: &mut Option<Box<Node<T>>>, new_node: Node<T>) {
    let Some(root_node) = root else {
        *root = Some(Box::new(new_node));
        return;
    };

    match new_node.value.cmp(&root_node.value) {
        Ordering::Greater => {
            inner_add(&mut root_node.right, new_node);
        }
        Ordering::Less => {
            inner_add(&mut root_node.left, new_node);
        }
        Ordering::Equal => {
            return;
        }
    };
}
