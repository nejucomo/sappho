use std::fmt::Formatter;

use crate::position::Position;
use crate::{Error, Result, Unparse};

pub struct Stream<'a, 'b> {
    optf: Option<&'a mut Formatter<'b>>,
    pos: Position,
}

impl<'a, 'b> Stream<'a, 'b> {
    pub fn new(f: &'a mut Formatter<'b>, max_width: usize) -> Self {
        Stream {
            optf: Some(f),
            pos: Position::new(max_width),
        }
    }

    pub fn write<U>(&mut self, value: U) -> Result<bool>
    where
        U: Unparse,
    {
        value.unparse(self)
    }

    pub fn trial_write<U>(&self, value: U) -> Result<bool>
    where
        U: Unparse,
    {
        let mut ts = self.trial_stream();
        match ts.write(value) {
            Err(Error::Wrapped) => Ok(true),
            other => other,
        }
    }

    fn trial_stream(&self) -> Self {
        Stream {
            optf: None,
            pos: self.pos,
        }
    }

    pub fn indent(&mut self) {
        self.pos.indent();
    }

    pub fn dedent(&mut self) {
        self.pos.dedent();
    }

    pub(crate) fn write_str(&mut self, s: &str) -> Result<bool> {
        let mut w = false;
        for (i, chunk) in s.split('\n').enumerate() {
            if i > 0 {
                w |= self.write_str_raw("\n")?;
                for _ in 0..self.pos.indentation_column() {
                    w |= self.write_str_raw(" ")?;
                }
            }
            w |= self.write_str_raw(chunk)?;
        }
        Ok(w)
    }

    fn write_str_raw(&mut self, s: &str) -> Result<bool> {
        let wrapped = self.pos.track_str(s);
        if let Some(f) = self.optf.as_mut() {
            f.write_str(s)?;
            Ok(wrapped)
        } else if wrapped {
            // In trial mode, we use err to terminate early:
            Err(Error::Wrapped)
        } else {
            Ok(false)
        }
    }
}
