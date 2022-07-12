use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};

/// A literal value, such as `3.1415`.
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Literal {
    /// A literal number value, such as `42`.
    Num(f64),
}

impl DisplayDepth for Literal {
    fn fmt_depth(&self, f: &mut Formatter, _depth: usize) -> FmtResult {
        use std::fmt::Display;
        use Literal::*;

        match self {
            Num(x) => x.fmt(f),
        }
    }
}
