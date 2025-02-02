use itertools::Itertools;
use itertools::Position::{Last, Only};

use crate::innernode::InnerNode;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::{IntoNode, Node};

pub(crate) type NodeSeparatedSeq = SeparatedSeq<Node>;

/// A separated sequence which can optionally be wrapped after each separator
#[derive(Clone, Debug)]
pub struct SeparatedSeq<X> {
    items: Vec<X>,
    sep: &'static str,
}

impl<X> SeparatedSeq<X> {
    /// Construct a new separated sequence from an iterator
    pub fn new<I>(items: I, sep: &'static str) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        SeparatedSeq {
            items: items.into_iter().collect(),
            sep,
        }
    }

    /// Map the elements to a new type
    pub fn map<F, Y>(self, f: F) -> SeparatedSeq<Y>
    where
        F: Fn(X) -> Y,
    {
        SeparatedSeq::new(self.items.into_iter().map(f), self.sep)
    }
}

impl<X> IntoNode for SeparatedSeq<X>
where
    X: IntoNode,
{
    fn into_node(self) -> Node {
        InnerNode::SeparatedSeq(self.map(X::into_node)).into_node()
    }
}

impl LegibleDisplay for NodeSeparatedSeq {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

impl WrappableDisplay for NodeSeparatedSeq {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        for (pos, x) in self.items.iter().with_position() {
            stream.write(x)?;
            if !matches!(pos, Last | Only) {
                stream.write(self.sep)?;
            }
            stream.write_joint(" ", wrap)?;
        }
        Ok(())
    }
}
