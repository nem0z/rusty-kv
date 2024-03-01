
struct Node<T> {
    key: u64,
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl <T> Node<T> {
    pub fn new(key: u64, val: T) -> Self {
        return Node{ key, value: val, left: None, right: None};
    }
}

pub struct Tree<T> {
    head: Option<Box<Node<T>>>
}

#[allow(dead_code)]
impl <T> Tree<T> {
    pub fn new() -> Self {
        return Tree{ head: None};
    }

    pub fn set(&mut self, key: u64, val: T) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.key == key {
                node.value = val;
                return;
            }

            if key > node.key {
                current = &mut node.right;
            } else {
                current = &mut node.left;
            }
        }

        *current = Some(Box::new(Node::new(key, val)));
    }

    pub fn get(&self, key: u64) -> Option<&T> {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.key == key {
                return Some(&node.value);
            }

            if key > node.key {
                current = &node.right;
            } else {
                current = &node.left;
            }
        }

        return None;
    }
}

