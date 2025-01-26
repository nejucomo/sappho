use std::fmt::Formatter;

use derive_new::new;

use crate::position::Position;
use crate::{Result, Unparse};

#[derive(new)]
#[new(visibility = "pub(crate)")]
pub struct Stream<'a, 'b> {
    optf: Option<&'a mut Formatter<'b>>,
    pos: Position,
}

impl<'a, 'b> Stream<'a, 'b> {
    pub fn write<U>(&mut self, value: U) -> Result<()>
    where
        U: Unparse,
    {
        value.unparse(self)
    }

    #[allow(dead_code)]
    pub(crate) fn wrap_trial(&self) -> Self {
        Stream {
            optf: None,
            pos: self.pos,
        }
    }

    pub(crate) fn write_str(&mut self, s: &str) -> Result<()> {
        self.optf.as_mut().map(|f| f.write_str(s)).transpose()?;
        self.pos.track_str(s)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn newline(&mut self, indent: bool) -> Result<()> {
        self.pos.indent_inc(indent);
        self.write_str("\n")?;
        for _ in 0..self.pos.indent_level() {
            for _ in 0..self.pos.indentation_size() {
                self.write_str(" ")?;
            }
        }
        Ok(())
    }
}
