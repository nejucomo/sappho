use crate::indentstream::IndentStream;
use crate::ldisp::LegibleDisplay;
use crate::position::Position;

pub(crate) trait Stream: Sized {
    type Error;

    fn write<L>(&mut self, value: L) -> Result<(), Self::Error>
    where
        L: LegibleDisplay,
    {
        value.write_to_stream(self)
    }

    fn indent(&mut self) -> IndentStream<Self> {
        IndentStream::from(self)
    }

    fn position(&self) -> Position;

    fn write_chunk(&mut self, chunk: &str) -> Result<(), Self::Error>;

    fn write_joint(&mut self, j: &str, wrap: bool) -> Result<(), Self::Error> {
        if wrap {
            self.write_newline()
        } else {
            self.write_chunk(j)
        }
    }

    fn write_newline(&mut self) -> Result<(), Self::Error>;
}
