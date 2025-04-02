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

impl<T> List<T> {}
