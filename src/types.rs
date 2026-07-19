pub struct Node<T> {
    value: T,
    right: Option<Box<Self>>,
    left: Option<Box<Self>>,
}
