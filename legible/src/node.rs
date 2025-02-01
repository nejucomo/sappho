use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::{Envelope, IntoNode, Sequence};

/// The pivotal type for [Legible](crate::Legible) which specifies a flexible layout textual representation
#[derive(Debug)]
pub enum Node {
    /// Single line or sub-lines of text which never wrap internally
    Text(String),
    /// A head, body, and optional tail where the body is indented when wrapped
    Envelope(Envelope),
    /// A sequence of items at the same indentation when wrapped
    Sequence(Sequence),
}

impl IntoNode for String {
    fn into_node(self) -> Node {
        Node::Text(self)
    }
}

impl<'a> IntoNode for &'a str {
    fn into_node(self) -> Node {
        self.to_string().into_node()
    }
}

impl IntoNode for Node {
    fn into_node(self) -> Node {
        self
    }
}

impl IntoNode for Envelope {
    fn into_node(self) -> Node {
        Node::Envelope(self)
    }
}

impl IntoNode for Sequence {
    fn into_node(self) -> Node {
        Node::Sequence(self)
    }
}

impl LegibleDisplay for Node {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        use Node::*;

        match self {
            Text(x) => x.write_to_stream(stream),
            Envelope(x) => x.write_to_stream(stream),
            Sequence(x) => x.write_to_stream(stream),
        }
    }
}
