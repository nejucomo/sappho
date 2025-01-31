use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::{Text, TextError};

/// Text which is replaced with newline when wrapping
#[derive(Debug)]
pub(crate) struct Joint(Text<'static>);

impl TryFrom<&'static str> for Joint {
    type Error = TextError<'static>;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        Text::try_from(s).map(Joint)
    }
}

impl WrappableDisplay for Joint {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        stream.write(if wrap { "\n" } else { self.0.as_ref() })
    }
}
