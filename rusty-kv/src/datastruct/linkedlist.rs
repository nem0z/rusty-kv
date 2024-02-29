struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}
#[allow(dead_code)]
impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        return Node { value: val, next: None };
    }

    pub fn new_with_next(val: T, next: Option<Box<Node<T>>>) -> Self {
        return Node { value: val, next: next };
    }
}

pub struct Linkedlist<T> {
    head: Option<Box<Node<T>>>
}
