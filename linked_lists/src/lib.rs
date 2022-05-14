
#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd> LinkedList<T> {
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
        for i in 0..index {
            match current.0 {
                Some((_, ref mut child)) => current = child,
                None => break,
            }
        }
        let t = current.0.take();
        current.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn insert_sorted(&mut self, data: T) {
        let mut current: &mut LinkedList<T>;
        match self.0 {
            Some((val, ref mut child)) => {
                if data.lt(&val) {
                    self.push_front(data);
                    return;
                } else {
                    current = child;
                }
            },
            None => self.push_front(data),
        }
        
        loop {
            match current.0 {
                Some((val, ref mut child)) => {
                    if data.lt(val) {
                        let t = current.0.take();
                        current.0 = Some((data, Box::new(LinkedList(t))));
                        return;
                    } else {
                        current = child;
                    }
                },
                None => return,
            }
        }
    }

    pub fn get_at(&mut self, index: usize) -> Option<(T, Box<LinkedList<T>>)> {
        let mut current: &mut LinkedList<T> = self;
        for i in 0..index {
            match current.0 {
                Some((_, ref mut child)) => current = child,
                None => break,
            }
        }
        let v = current.0;
        match v {
            Some(v) => Some(v),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut ll = LinkedList::new();
        ll.push_back(3);
        ll.push_back(12);
        ll.push_back(9);

        assert_eq!(ll, 0);
    }
}
