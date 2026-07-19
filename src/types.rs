pub struct Node<T> {
    pub value: T,
    pub right: Option<Box<Self>>,
    pub left: Option<Box<Self>>,
}

pub trait RbtreeSetExt<T: Ord> {
    fn add(&mut self, value: T);
    fn contains(&self, value: T) -> bool;
    fn remove(&self, value: T) -> bool;
    fn size(&self) -> usize;
}

pub struct RBTreeSet<T> {
    pub(crate) root: Option<Box<Node<T>>>,
}
