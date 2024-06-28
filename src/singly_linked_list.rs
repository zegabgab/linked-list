pub struct SinglyLinkedList<T> {
    first: Option<SinglyLinkedNode<T>>,
}

struct SinglyLinkedNode<T> {
    next: Option<Box<SinglyLinkedNode<T>>>,
    value: T,
}

impl<T> SinglyLinkedNode<T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        match index {
            0 => Some(&self.value),
            index => self.next.as_ref()?.get(index - 1),
        }
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList { first: None }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.first.as_ref()?.get(index)
    }
}

impl<A> FromIterator<A> for SinglyLinkedList<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        todo!()
    }
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

mod tests {
    #[test]
    fn out_of_bounds() {
        use super::*;

        let list = SinglyLinkedList {
            first: Some(SinglyLinkedNode {
                next: Some(Box::new(SinglyLinkedNode {
                    next: None,
                    value: 69,
                })),
                value: 420,
            }),
        };
        assert_eq!(list.get(2), None);
    }

    #[test]
    fn sixty_nine() {
        use super::*;

        let list = SinglyLinkedList {
            first: Some(SinglyLinkedNode {
                next: Some(Box::new(SinglyLinkedNode {
                    next: None,
                    value: 69,
                })),
                value: 420,
            }),
        };
        assert_eq!(list.get(1), Some(&69));
    }
}
