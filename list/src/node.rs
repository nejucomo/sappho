use crate::List;

#[derive(Debug)]
pub(crate) struct Node<T> {
    elem: T,
    tail: List<T>,
}
