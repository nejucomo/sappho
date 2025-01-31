use itertools::Itertools;

use crate::joint::Joint;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::{Node, Text};

/// A sequence of items at the same indentation when wrapped
#[derive(Debug)]
pub struct Sequence<'a> {
    items: Vec<Node<'a>>,
    separator: Text<'a>,
    terminal: bool,
    joint: Joint,
}

impl<'a> LegibleDisplay for Sequence<'a> {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

/// We layout unwrapped if we can fit on the line or we have 1 or fewer items
impl<'a> WrappableDisplay for Sequence<'a> {
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
