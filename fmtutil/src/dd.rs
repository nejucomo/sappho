use std::fmt::Display;
pub use std::fmt::{Formatter, Result as FmtResult};

pub trait DisplayDepth {
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult;
}

pub fn indent(f: &mut Formatter, depth: usize) -> FmtResult {
    for _ in 0..depth {
        write!(f, "  ")?;
    }
    Ok(())
}

impl DisplayDepth for String {
    fn fmt_depth(&self, f: &mut Formatter, _depth: usize) -> FmtResult {
        self.fmt(f)
    }
}

impl<X> DisplayDepth for Box<X>
where
    X: DisplayDepth,
{
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        self.as_ref().fmt_depth(f, depth)
    }
}
