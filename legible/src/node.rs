use crate::innernode::InnerNode;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::IntoNode;

/// The pivotal type for [Legible](crate::Legible) which specifies a flexible layout textual representation
#[derive(Clone, Debug)]
pub struct Node(InnerNode);

impl IntoNode for Node {
    fn into_node(self) -> Node {
        self
    }
}

impl IntoNode for InnerNode {
    fn into_node(self) -> Node {
        Node(self)
    }
}

impl<X> FromIterator<X> for Node
where
    X: IntoNode,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        Node(iter.into_iter().collect())
    }
}

impl LegibleDisplay for Node {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.0.write_to_stream(stream)
    }
}
