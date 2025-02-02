use crate::innernode::InnerNode;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::writestr::WriteStr;
use crate::{IntoNode, Node, SeparatedSeq, Text};

pub(crate) type NodeBracketSeq = BracketSeq<Text, Text, Node>;

/// A bracketed comma-separated sequence
#[derive(Clone, Debug)]
pub struct BracketSeq<Open, Close, Item> {
    brackets: (Open, Close),
    sepseq: SeparatedSeq<Item>,
}

impl<O, C, X> BracketSeq<O, C, X> {
    /// Create a new BracketSeq from an iterator
    pub fn new<I>(brackets: (O, C), sep: &'static str, items: I) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        BracketSeq {
            brackets,
            sepseq: SeparatedSeq::new(items, sep),
        }
    }
}

impl<O, C, X> IntoNode for BracketSeq<O, C, X>
where
    Text: From<O> + From<C>,
    X: IntoNode,
{
    fn into_node(self) -> Node {
        let (open, close) = self.brackets;
        let open = Text::from(open);
        let close = Text::from(close);
        InnerNode::BracketSeq(BracketSeq {
            brackets: (open, close),
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
        stream.write(open)?;
        stream.write_joint(" ", wrap)?;

        stream.indent();
        stream.write_wrap(&self.sepseq, wrap)?;
        stream.dedent();

        stream.write_joint(" ", wrap)?;
        stream.write(close)?;
        Ok(())
    }
}
