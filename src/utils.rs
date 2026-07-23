use std::{cmp::Ordering, fmt::Display};

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub right: Option<Box<Self>>,
    pub left: Option<Box<Self>>,
}

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

pub fn inner_size<T>(root: &Option<Box<Node<T>>>) -> usize {
    if let Some(root_node) = root {
        1 + inner_size(&root_node.left) + inner_size(&root_node.right)
    } else {
        0
    }
}
