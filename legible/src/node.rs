use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::{Envelope, IntoNode, Sequence, Text, TextError};

/// The pivotal type for [Legible](crate::Legible) which specifies a flexible layout textual representation
#[derive(Debug)]
pub enum Node<'a> {
    /// Single line or sub-lines of text which never wrap internally
    Text(Text<'a>),
    /// A head, body, and optional tail where the body is indented when wrapped
    Envelope(Envelope<'a>),
    /// A sequence of items at the same indentation when wrapped
    Sequence(Sequence<'a>),
}

impl<'a> Node<'a> {}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = TextError<'a>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Text::try_from(s).map(Node::Text)
    }
}

impl<'a> TryFrom<String> for Node<'a> {
    type Error = TextError<'a>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Text::try_from(s).map(Node::Text)
    }
}

impl<'a> IntoNode<'a> for Node<'a> {
    fn into_node(self) -> Node<'a> {
        self
    }
}

impl<'a> IntoNode<'a> for Text<'a> {
    fn into_node(self) -> Node<'a> {
        Node::Text(self)
    }
}

impl<'a> IntoNode<'a> for Envelope<'a> {
    fn into_node(self) -> Node<'a> {
        Node::Envelope(self)
    }
}

impl<'a> IntoNode<'a> for Sequence<'a> {
    fn into_node(self) -> Node<'a> {
        Node::Sequence(self)
    }
}

impl<'a> LegibleDisplay for Node<'a> {
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
