mod implsequences;
mod impltext;

use crate::Node;

/// Convert into a [Node]
pub trait IntoNode {
    /// Convert into a [Node]
    fn into_node(self) -> Node;
}

impl<'a, T> IntoNode for &'a Box<T>
where
    &'a T: IntoNode,
{
    fn into_node(self) -> Node {
        self.as_ref().into_node()
    }
}
