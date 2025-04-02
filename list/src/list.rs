use std::rc::Rc;

use crate::node::Node;

#[derive(Debug)]
pub struct List<T>(Option<Rc<Node<T>>>);

impl<T> Default for List<T> {
    fn default() -> Self {
        List(None)
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        List(self.0.clone())
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> + Into<Self> {
        ListIter { listptr: self }
    }

    pub fn prepend(self, elem: T) -> Self {
        List(Some(Rc::new(Node { elem, tail: self })))
    }
}

#[derive(Debug)]
pub struct ListIter<'a, T> {
    listptr: &'a List<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.listptr.0.as_ref().map(|rcn| {
            self.listptr = &rcn.tail;
            &rcn.elem
        })
    }
}

impl<T> From<ListIter<'_, T>> for List<T> {
    fn from(li: ListIter<'_, T>) -> Self {
        li.listptr.clone()
    }
}
