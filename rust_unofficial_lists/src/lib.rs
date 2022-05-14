use std::mem;
use std::vec::Vec;

pub struct List<T> {
    head: Link<T>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = current {
            current = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T: Copy> List<T> {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn from(array: &mut Vec<T>) -> Self {
        let a = mem::take(array);
        let mut l = List::new();
        for i in 0..a.len() {
            l.push(a[i]);
        }
        *array = a;
        l
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        list.push(5);
        list.push(6);
        list.push(7);
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(5));
        let mut v = vec![1, 2, 3];
        list = List::from(&mut v);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
