use std::fmt::Formatter;

use derive_new::new;

use crate::position::Position;
use crate::{Result, Stream};

#[derive(new)]
#[new(visibility = "pub(crate)")]
pub struct FmtPos<'a, 'b> {
    f: &'a mut Formatter<'b>,
    pos: Position,
}

impl<'a, 'b> Stream for FmtPos<'a, 'b> {
    fn write_str(&mut self, s: &str) -> Result<()> {
        Formatter::write_str(&mut self.f, s)?;
        self.pos.write_str(s)?;
        Ok(())
    }

    fn position(&self) -> Position {
        self.pos
    }
}
