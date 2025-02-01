use derive_new::new;

use crate::innernode::InnerNode;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::{IntoNode, Node};

pub(crate) type NodeHeadAndTail = HeadAndTail<Box<Node>, Box<Node>>;

/// A head construct with an optionally indented tail with `sep` separator
#[derive(Clone, Debug, new)]
pub struct HeadAndTail<H, T> {
    head: H,
    sep: &'static str,
    tail: T,
}

impl<H, T> IntoNode for HeadAndTail<H, T>
where
    H: IntoNode,
    T: IntoNode,
{
    fn into_node(self) -> Node {
        InnerNode::HeadAndTail(HeadAndTail::new(
            Box::new(self.head.into_node()),
            self.sep,
            Box::new(self.tail.into_node()),
        ))
        .into_node()
    }
}

impl LegibleDisplay for NodeHeadAndTail {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

impl WrappableDisplay for NodeHeadAndTail {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        stream.write(&self.head)?;
        let mut substream = stream.indent();
        substream.write_joint(self.sep, wrap)?;
        substream.write(&self.tail)?;
        Ok(())
    }
}
