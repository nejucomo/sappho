use crate::indentation::IndentationDelta::{Dedent, Indent};
use crate::joint::Joint;
use crate::ldisp::LegibleDisplay;
use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::{IntoNode, Node};

/// A head, body, and optional tail where the body is indented when wrapped
#[derive(Debug)]
pub struct Envelope {
    head: Box<Node>,
    body: Box<Node>,
    optail: Option<Box<Node>>,
}

impl Envelope {
    /// Construct a new envelope with a tail
    pub fn new_with_tail<A, B, C>(head: A, body: B, tail: C) -> Self
    where
        A: IntoNode,
        B: IntoNode,
        C: IntoNode,
    {
        Envelope {
            head: Box::new(head.into_node()),
            body: Box::new(body.into_node()),
            optail: Some(Box::new(tail.into_node())),
        }
    }
}

impl LegibleDisplay for Envelope {
    fn write_to_stream<S>(&self, stream: &mut S) -> Result<(), S::Error>
    where
        S: Stream,
    {
        self.write_to_stream_maybe_wrapped(stream)
    }
}

impl WrappableDisplay for Envelope {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        let joint = Joint::try_from(" ").unwrap();

        stream.write(&self.head)?;
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
