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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_linkedlist() {
        let list: Linkedlist<i32> = Linkedlist::new();
        assert!(list.head.is_none());
    }

    #[test]
    fn test_new_linkedlist_from_values() {
        let list: Linkedlist<i32> = Linkedlist::new_from_values(&[8, 4, 3, 12, 11]);
        assert_eq!(list.to_string(), "[11, 12, 3, 4, 8]");
    }

    #[test]
    fn test_push() {
        let mut list = Linkedlist::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        assert_eq!(list.to_string(), "[4, 3, 2, 1]");
    }

    #[test]
    fn test_is_sorted() {
        let sorted_list = Linkedlist::new_from_values(&[3, 2, 1]);
        let unsorted_list = Linkedlist::new_from_values(&[3, 1, 2]);
        assert!(sorted_list.is_sorted());
        assert!(!unsorted_list.is_sorted());
    }

    #[test]
    fn test_push_sort() {
        let mut list = Linkedlist::new_from_values(&[92, 23, 11]);
        list.push_sort(46);
        list.push_sort(8);
        list.push_sort(111);
        assert!(list.is_sorted());
    }
}
