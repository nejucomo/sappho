use crate::stream::Stream;
use crate::wrappable::WrappableDisplay;
use crate::writestr::WriteStr;

/// Text which is replaced with newline when wrapping
#[derive(Copy, Clone, Debug, derive_more::From)]
pub(crate) struct Joint(&'static str);

impl WrappableDisplay for Joint {
    fn write_to_stream_with_wrap<W>(
        &self,
        stream: &mut Stream<W>,
        wrap: bool,
    ) -> Result<(), W::Error>
    where
        W: WriteStr,
    {
        if wrap {
            stream.write_newline()
        } else {
            stream.write(self.0)
        }
    }
}
