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

    pub fn push_sort(&mut self, val: T) where T: std::cmp::PartialOrd {
        let mut current = &mut self.head;
        while let Some(c) = current {
            if c.value >= val {
                c.next = Some(std::mem::replace(c, Box::new(Node::new(val))));
                return;
            }
            current = &mut c.next;
        }
        *current = Some(Box::new(Node::new(val)));
    }

    pub fn is_sorted(&self) -> bool where T: std::cmp::PartialOrd {
        let mut prev = match &self.head {
            Some(node) => node,
            None => return true,
        };

        while let Some(current) = &prev.next {
            if current.value < prev.value {
                return false;
            }

            prev = current;
        }

        return true;
    }
}
