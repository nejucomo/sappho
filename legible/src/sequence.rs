use itertools::Itertools;

use crate::joint::Joint;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::{IntoNode, Node, Text};

/// A sequence of items at the same indentation when wrapped
#[derive(Debug)]
pub struct Sequence<'s> {
    items: Vec<Node<'s>>,
    separator: Text<'s>,
    terminal: bool,
    joint: Joint,
}

impl<'s> Sequence<'s> {
    /// Create a new sequence with the given separator
    pub fn new_separated<I, X>(items: I, separator: &'static str) -> Self
    where
        I: IntoIterator<Item = X>,
        X: IntoNode<'s>,
    {
        Sequence {
            items: items.into_iter().map(X::into_node).collect(),
            separator: Text::try_from(separator).unwrap(),
            terminal: false,
            joint: Joint::try_from(" ").unwrap(),
        }
    }
}

impl<'s> LegibleDisplay for Sequence<'s> {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

/// We layout unwrapped if we can fit on the line or we have 1 or fewer items
impl<'s> WrappableDisplay for Sequence<'s> {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        use itertools::Position::{Last, Only};

        for (pos, item) in self.items.iter().with_position() {
            stream.write(item)?;
            if matches!(pos, Last | Only) {
                if self.terminal {
                    stream.write(&self.separator)?;
                }
            } else {
                stream.write_with_wrap(&self.joint, wrap)?;
            }
        }
        Ok(())
    }
}
