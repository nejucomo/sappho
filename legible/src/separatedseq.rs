use itertools::Itertools;
use itertools::Position::{Last, Only};

use crate::innernode::InnerNode;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::writestr::WriteStr;
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

    /// Are there no elements?
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
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
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

impl WrappableDisplay for NodeSeparatedSeq {
    fn write_to_stream_with_wrap<W>(
        &self,
        stream: &mut Stream<W>,
        wrap: bool,
    ) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        let wrap = wrap || self.sep.contains('\n');
        for (pos, x) in self.items.iter().with_position() {
            stream.write(x)?;
            if !matches!(pos, Last | Only) {
                stream.write_joint(self.sep, wrap)?;
            }
        }
        Ok(())
    }
}
