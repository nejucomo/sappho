use crate::innernode::InnerNode;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::writestr::WriteStr;
use crate::{IntoNode, Node, SeparatedSeq};

pub(crate) type NodeBracketSeq = BracketSeq<Node>;

/// A bracketed comma-separated sequence
#[derive(Clone, Debug)]
pub struct BracketSeq<Item> {
    brackets: Brackets,
    sepseq: SeparatedSeq<Item>,
}

type Brackets = (&'static str, &'static str);

impl<X> BracketSeq<X> {
    /// Create a new BracketSeq from an iterator
    pub fn new<I>(brackets: Brackets, sep: &'static str, items: I) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        BracketSeq {
            brackets,
            sepseq: SeparatedSeq::new(items, sep),
        }
    }
}

impl<X> IntoNode for BracketSeq<X>
where
    X: IntoNode,
{
    fn into_node(self) -> Node {
        InnerNode::BracketSeq(BracketSeq {
            brackets: self.brackets,
            sepseq: self.sepseq.map(X::into_node),
        })
        .into_node()
    }
}

impl LegibleDisplay for NodeBracketSeq {
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

impl WrappableDisplay for NodeBracketSeq {
    fn write_to_stream_with_wrap<W>(
        &self,
        stream: &mut Stream<W>,
        wrap: bool,
    ) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        let (open, close) = &self.brackets;
        if self.sepseq.is_empty() {
            stream.write(open.trim())?;
            stream.write(close.trim())?;
        } else {
            let wrap = wrap || open.contains('\n') || close.contains('\n');
            stream.indent(wrap);
            stream.write_joint(open, wrap)?;

            stream.write_wrap(&self.sepseq, wrap)?;

            stream.dedent(wrap);
            stream.write_joint(close, wrap)?;
        }
        Ok(())
    }
}
