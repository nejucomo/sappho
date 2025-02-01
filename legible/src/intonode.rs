use crate::Node;

/// Convert into a [Node]
pub trait IntoNode {
    /// Convert into a [Node]
    fn into_node(self) -> Node;
}
