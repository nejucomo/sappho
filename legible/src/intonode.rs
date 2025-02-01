use crate::Node;

/// Convert into a [Node]
pub trait IntoNode {
    /// Convert into a [Node]
    fn into_node(self) -> Node;
}

impl<T, const K: usize> IntoNode for [T; K]
where
    T: IntoNode,
{
    fn into_node(self) -> Node {
        self.into_iter().map(T::into_node).collect()
    }
}

impl<A, B> IntoNode for (A, B)
where
    A: IntoNode,
    B: IntoNode,
{
    fn into_node(self) -> Node {
        let (a, b) = self;
        [a.into_node(), b.into_node()].into_node()
    }
}

impl<A, B, C> IntoNode for (A, B, C)
where
    A: IntoNode,
    B: IntoNode,
    C: IntoNode,
{
    fn into_node(self) -> Node {
        let (a, b, c) = self;
        (a, (b, c)).into_node()
    }
}

impl<A, B, C, D> IntoNode for (A, B, C, D)
where
    A: IntoNode,
    B: IntoNode,
    C: IntoNode,
    D: IntoNode,
{
    fn into_node(self) -> Node {
        let (a, b, c, d) = self;
        (a, (b, c, d)).into_node()
    }
}
