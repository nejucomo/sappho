use itertools::Itertools;
use itertools::Position::{Last, Only};

use crate::innernode::InnerNode;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::{IntoNode, Node, Text};

pub(crate) type NodeBracketSeq = BracketSeq<Text, Text, Node>;

/// A bracketed comma-separated sequence
#[derive(Clone, Debug)]
pub struct BracketSeq<Open, Close, Item> {
    open: Open,
    close: Close,
    items: Vec<Item>,
}

impl<O, C, X> IntoNode for BracketSeq<O, C, X>
where
    Text: From<O> + From<C>,
    X: IntoNode,
{
    fn into_node(self) -> Node {
        InnerNode::BracketSeq(BracketSeq {
            open: Text::from(self.open),
            close: Text::from(self.close),
            items: self.items.into_iter().map(X::into_node).collect(),
        })
        .into_node()
    }
}

impl LegibleDisplay for NodeBracketSeq {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

impl WrappableDisplay for NodeBracketSeq {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        stream.write(&self.open)?;
        stream.write_joint(" ", wrap)?;
        let mut substream = stream.indent();
        for (pos, x) in self.items.iter().with_position() {
            substream.write(x)?;
            if !matches!(pos, Last | Only) {
                substream.write(",")?;
            }
            substream.write_joint(" ", wrap)?;
        }

        let stream = substream.dedent();
        stream.write_joint(" ", wrap)?;
        stream.write(&self.close)?;
        Ok(())
    }
}
