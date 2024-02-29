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

impl<T: fmt::Display> fmt::Display for Linkedlist<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Err(e) = write!(f, "[") {
            return Err(e);
        }

        if let Some(head) = &self.head {
            if let Err(e) = write!(f, "{}", head.value) {
                return Err(e);
            }

            let mut current = head;
            while let Some(next) = &current.next {
                if let Err(e) = write!(f, ", {}", next.value) {
                    return Err(e);
                }
                current = next;
            }
        }

        return write!(f, "]");

    }
}

#[allow(dead_code)]
impl<T> Linkedlist<T> {
    pub fn new() -> Self {
        return Linkedlist { head: None }
    }

    pub fn new_from_values(values: &[T]) -> Self where T: Clone {
        let mut prev: Option<Box<Node<T>>> = None;

        for val in values.iter().cloned() {
            let node: Node<T> = Node::new_with_next(val, prev.take());
            prev = Some(Box::new(node));
        }

        return Linkedlist { head: prev };
}

    pub fn push(&mut self, val: T) {
        let node = Node::new_with_next(val, self.head.take());
        self.head = Some(Box::new(node));
    }

