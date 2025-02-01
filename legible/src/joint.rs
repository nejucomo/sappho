use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;

/// Text which is replaced with newline when wrapping
#[derive(Debug)]
pub(crate) struct Joint(&'static str);

impl From<&'static str> for Joint {
    fn from(s: &'static str) -> Self {
        Joint(s)
    }
}

impl WrappableDisplay for Joint {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        stream.write(if wrap { "\n" } else { self.0 })
    }
}
