use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;

/// Text which is replaced with newline when wrapping
#[derive(Copy, Clone, Debug, derive_more::From)]
pub(crate) struct Joint(&'static str);

impl WrappableDisplay for Joint {
    fn write_to_stream_with_wrap<S>(&self, stream: &mut S, wrap: bool) -> Result<(), S::Error>
    where
        S: Stream,
    {
        stream.write(if wrap { "\n" } else { self.0 })
    }
}
