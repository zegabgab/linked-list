use std::rc::Rc;

pub enum Cons<T> {
    Empty,
    List(T, Rc<Cons<T>>),
}

impl<T> Cons<T> {
    pub fn new() -> Cons<T> {
        Cons::Empty
    }

    pub fn head(&self) -> Option<&T> {
        match self {
            Cons::Empty => None,
            Cons::List(value, _) => Some(value),
        }
    }

    pub fn tail(&self) -> Option<&Cons<T>> {
        match self {
            Cons::Empty => None,
            Cons::List(_, tail) => Some(tail),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match index {
            0 => self.head(),
            index => self.tail()?.get(index - 1),
        }
    }

    pub fn prepend(value: T, list: Rc<Cons<T>>) -> Cons<T> {
        Cons::List(value, list)
    }
}

impl<T> Default for Cons<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConsIterator<'a, T> {
    cons: &'a Cons<T>,
}

impl<'a, T> Iterator for ConsIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cons {
            Cons::Empty => None,
            Cons::List(head, tail) => {
                self.cons = tail;
                Some(head)
            },
        }
    }
}

impl<'a, T> IntoIterator for &'a Cons<T> {
    type Item = &'a T;

    type IntoIter = ConsIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ConsIterator { cons: self }
    }
}
