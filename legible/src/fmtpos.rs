use std::fmt::Formatter;

use crate::position::Position;
use crate::stream::Stream;

pub(crate) struct FmtPos<'a, 'b> {
    f: &'a mut Formatter<'b>,
    pos: Position,
}

impl<'a, 'b> FmtPos<'a, 'b> {
    pub(crate) fn new(f: &'a mut Formatter<'b>, max_width: usize) -> Self {
        let pos = Position::new(max_width);
        FmtPos { f, pos }
    }
}

impl Stream for FmtPos<'_, '_> {
    type Error = std::fmt::Error;

    fn position(&self) -> Position {
        self.pos
    }

    fn write_chunk(&mut self, chunk: &str) -> Result<(), Self::Error> {
        self.f.write_str(chunk)?;
        self.pos.track_chunk(chunk);
        Ok(())
    }

    fn write_newline(&mut self) -> Result<(), Self::Error> {
        self.f.write_str("\n")?;
        self.pos.reset_column();
        Ok(())
    }
}
