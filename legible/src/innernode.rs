use crate::bracketed::NodeBracketSeq;
use crate::headandtail::NodeHeadAndTail;
use crate::ldisp::LegibleDisplay;
use crate::separatedseq::NodeSeparatedSeq;
use crate::stream::Stream;
use crate::writestr::WriteStr;
use crate::{IntoNode, Node, Text};

#[derive(Clone, Debug)]
pub(crate) enum InnerNode {
    Text(Text),
    Sequence(Vec<Node>),
    SeparatedSeq(NodeSeparatedSeq),
    BracketSeq(NodeBracketSeq),
    HeadAndTail(NodeHeadAndTail),
}

impl<X> FromIterator<X> for InnerNode
where
    X: IntoNode,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        InnerNode::Sequence(iter.into_iter().map(X::into_node).collect())
    }
}

impl LegibleDisplay for InnerNode {
    fn write_to_stream<W>(&self, stream: &mut Stream<W>) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        use InnerNode::*;

        match self {
            Text(x) => x.write_to_stream(stream),
            Sequence(v) => {
                for x in v {
                    x.write_to_stream(stream)?;
                }
                Ok(())
            }
            SeparatedSeq(x) => x.write_to_stream(stream),
            BracketSeq(x) => x.write_to_stream(stream),
            HeadAndTail(x) => x.write_to_stream(stream),
        }
    }
}
