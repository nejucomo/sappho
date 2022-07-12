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
