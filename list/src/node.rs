use crate::List;

#[derive(Debug, PartialEq)]
pub(crate) struct Node<T> {
    elem: T,
    tail: List<T>,

    // We cache the list length:
    length: usize,
}

impl<T> Node<T> {
    pub(crate) fn new(elem: T, tail: List<T>) -> Self {
        let length = tail.length();

        Node { elem, tail, length }
    }

    pub(crate) fn length(&self) -> usize {
        self.length
    }

    pub(crate) fn elem(&self) -> &T {
        &self.elem
    }

    pub(crate) fn tail(&self) -> &List<T> {
        &self.tail
    }
}
