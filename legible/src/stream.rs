use itertools::Itertools;

use crate::indentation::IndentationDelta;
use crate::ldisp::LegibleDisplay;
use crate::position::Position;
use crate::wrappable::WrappableDisplay;

pub(crate) trait Stream: Sized {
    type Error;

    fn write<L>(&mut self, value: L) -> Result<(), Self::Error>
    where
        L: LegibleDisplay,
    {
        value.write_to_stream(self)
    }

    fn write_with_wrap<W>(&mut self, value: W, wrap: bool) -> Result<(), Self::Error>
    where
        W: WrappableDisplay,
    {
        value.write_to_stream_with_wrap(self, wrap)
    }

    fn indent(&mut self, delta: IndentationDelta) {
        self.position_mut().indentation.apply_delta(delta);
    }

    fn position(&self) -> Position;
    fn position_mut(&mut self) -> &mut Position;
    fn write_chunk(&mut self, chunk: &str) -> Result<(), Self::Error>;
    fn write_newline(&mut self) -> Result<(), Self::Error>;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        use itertools::Position::{Last, Middle};

        for (pos, chunk) in s.split('\n').with_position() {
            if matches!(pos, Middle | Last) {
                // Emit a newline:
                self.write_newline()?;
            }
            self.write_chunk(chunk)?;
        }
        Ok(())
    }
}
