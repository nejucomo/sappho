use crate::GenExpr;
use std::fmt;

/// Query expressions can read mutable memory, as in `$myvar`.
pub type QueryExpr = GenExpr<QueryEffects>;

/// The query effect reads mutable memory.
#[derive(Debug, PartialEq)]
pub enum QueryEffects {
    /// Inquire is the name of the `$myvar` effect syntax & semantics.
    Inquire(Box<GenExpr<QueryEffects>>),
}

impl fmt::Display for QueryEffects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use QueryEffects::*;

        match self {
            Inquire(x) => {
                write!(f, "$")?;
                x.fmt(f)
            }
        }
    }
}
