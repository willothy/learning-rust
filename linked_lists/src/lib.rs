#[derive(Debug, Clone)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd + Copy + Default> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn insert(&mut self, data: T, index: usize) {
        let mut current: &mut LinkedList<T> = self;
        for _ in 0..index {
            match current.0 {
                Some((_, ref mut child)) => current = child,
                None => break,
            }
        }
        let t = current.0.take();
        current.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn get_at(& self, index: usize) -> Option<T> {
        let mut current: & LinkedList<T> = self;
        for _ in 0..index {
            match current.0 {
                Some((_, ref child)) => current = child,
                None => break,
            }
        }

        match current.0.clone() {
            Some(v) => Some(v.0),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_linked_list() {
        let mut ll = LinkedList::new();
        ll.push_front(0);
        ll.push_front(5);
        ll.push_front(8);

        assert_eq!(ll.get_at(0), Some(8));
        assert_eq!(ll.get_at(1), Some(5));
        assert_eq!(ll.get_at(2), Some(0));
    }
}
