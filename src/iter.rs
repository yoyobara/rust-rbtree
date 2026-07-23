use crate::utils::Node;

pub struct Iter<'a, T: Ord> {
    pub stack: Vec<&'a Box<Node<T>>>,
}

impl<'a, T: Ord> Iter<'a, T> {
    pub fn push_left(&mut self, mut node: &'a Box<Node<T>>) {
        loop {
            self.stack.push(node);

            if let Some(left_child) = &node.left {
                node = &left_child;
            } else {
                break;
            }
        }
    }
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        if let Some(right_child) = &node.right {
            self.push_left(right_child);
        }

        Some(&node.value)
    }
}
