use std::fmt::{Formatter, Result};

pub trait DisplayDepth {
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> Result;
}

pub trait FormatterExt<'a> {
    fn indent(&'a mut self, depth: usize) -> Result;
}

impl<'a> FormatterExt<'a> for Formatter<'a> {
    fn indent(&'a mut self, depth: usize) -> Result {
        for _ in 0..depth {
            write!(self, "  ")?;
        }
        Ok(())
    }
}
