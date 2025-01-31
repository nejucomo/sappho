use crate::Node;

/// Convert into a [Node]
pub trait IntoNode<'a> {
    /// Convert into a [Node]
    fn into_node(self) -> Node<'a>;
}
