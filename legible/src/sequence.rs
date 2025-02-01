use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::{IntoNode, Joint, Node};

/// A sequence of items at the same indentation when wrapped
#[derive(Clone, Debug)]
pub struct Sequence(Vec<Node>);

impl Sequence {
    /// Construct a wrappable sequence of `separator`-separated items
    pub fn separated<I, X>(separator: &'static str, items: I) -> Self
    where
        I: IntoIterator<Item = X>,
        X: IntoNode,
    {
        itertools::intersperse(
            items.into_iter().map(X::into_node),
            (separator, Joint::from(" ")).into_node(),
        )
        .collect()
    }
}

impl LegibleDisplay for Sequence {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

/// We layout unwrapped if we can fit on the line or we have 1 or fewer items
impl WrappableDisplay for Sequence {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        for x in &self.0 {
            stream.write_with_wrap(x, wrap)?;
        }
        Ok(())
    }
}

impl<X> FromIterator<X> for Sequence
where
    X: IntoNode,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = X>,
    {
        Sequence(iter.into_iter().map(X::into_node).collect())
    }
}
