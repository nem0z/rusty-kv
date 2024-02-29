struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}
pub struct Linkedlist<T> {
    head: Option<Box<Node<T>>>
}
