use crate::GenExpr;
use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};

/// Query expressions can read mutable memory, as in `$myvar`.
pub type QueryExpr = GenExpr<QueryEffects>;

/// The query effect reads mutable memory.
#[derive(Debug, PartialEq)]
pub enum QueryEffects {
    /// Inquire is the name of the `$myvar` effect syntax & semantics.
    Inquire(Box<GenExpr<QueryEffects>>),
}

impl From<QueryEffects> for QueryExpr {
    fn from(x: QueryEffects) -> Self {
        GenExpr::Effect(x)
    }
}

impl DisplayDepth for QueryEffects {
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        use QueryEffects::*;

        match self {
            Inquire(x) => {
                write!(f, "$")?;
                x.fmt_depth(f, depth)
            }
        }
    }
}
