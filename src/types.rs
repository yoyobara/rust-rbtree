pub struct Node<T> {
    value: T,
    right: Option<Box<Self>>,
    left: Option<Box<Self>>,
}

pub trait RbtreeSetExt<T: Ord> {
    fn add(value: T);
    fn contains(value: T) -> bool;
    fn remove(value: T) -> bool;
    fn size() -> usize;
}

pub struct RBTreeSet<T> {
    root: Option<Node<T>>,
}
