use derive_new::new;

use crate::innernode::InnerNode;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::writestr::WriteStr;
use crate::{IntoNode, Node};

pub(crate) type NodeHeadAndTail = HeadAndTail<Box<Node>, Box<Node>>;

/// A head construct with an optionally indented tail with `sep` separator
#[derive(Clone, Debug, new)]
pub struct HeadAndTail<H, T> {
    /// The unindented head structure
    pub head: H,
    /// The separator
    pub sep: &'static str,
    /// the tail
    pub tail: T,
}

impl<H, T> IntoNode for HeadAndTail<H, T>
where
    H: IntoNode,
    T: IntoNode,
{
    fn into_node(self) -> Node {
        InnerNode::HeadAndTail(HeadAndTail {
            head: Box::new(self.head.into_node()),
            sep: self.sep,
            tail: Box::new(self.tail.into_node()),
        })
        .into_node()
    }
}

impl LegibleDisplay for NodeHeadAndTail {
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: crate::writestr::WriteStr,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

impl WrappableDisplay for NodeHeadAndTail {
    fn write_to_stream_with_wrap<W>(
        &self,
        stream: &mut Stream<W>,
        wrap: bool,
    ) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        let wrap = wrap || self.sep.contains('\n');
        stream.write(&self.head)?;
        stream.indent(wrap);
        stream.write_joint(self.sep, wrap)?;
        stream.write(&self.tail)?;
        stream.dedent(wrap);
        Ok(())
    }
}
