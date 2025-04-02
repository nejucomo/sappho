use crate::List;

#[derive(Debug)]
pub(crate) struct Node<T> {
    pub(crate) elem: T,
    pub(crate) tail: List<T>,
}
