use crate::indentation::IndentationDelta::{Dedent, Indent};
use crate::joint::Joint;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::Node;

/// A head, body, and optional tail where the body is indented when wrapped
#[derive(Debug)]
pub struct Envelope<'a> {
    header: Box<Node<'a>>,
    body: Box<Node<'a>>,
    optail: Option<Box<Node<'a>>>,
}

impl<'a> LegibleDisplay for Envelope<'a> {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

impl<'a> WrappableDisplay for Envelope<'a> {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        let joint = Joint::try_from(" ").unwrap();

        stream.write(&self.header)?;
        if wrap {
            stream.indent(Indent);
        }
        stream.write_with_wrap(&joint, wrap)?;
        stream.write(&self.body)?;
        if wrap {
            stream.indent(Dedent);
        }
        if let Some(tail) = self.optail.as_ref() {
            stream.write_with_wrap(joint, wrap)?;
            stream.write(tail)?;
        }
        Ok(())
    }
}
