use crate::GenExpr;
use std::fmt;

/// Proc expressions can cause mutations (in memory or I/O), as in `!launch_balloon`, as well as
/// causing [QueryEffects](crate::QueryEffects).
pub type ProcExpr = GenExpr<ProcEffects>;

/// A proc effect can either be a mutation or a query effect.
#[derive(Debug, PartialEq)]
pub enum ProcEffects {
    /// Inquire is identical to [QueryEffects::Inquire](crate::QueryEffects::Inquire).
    Inquire(Box<GenExpr<ProcEffects>>),

    /// Evoke a mutation, as in `!exit`.
    Evoke(Box<GenExpr<ProcEffects>>),
}

impl fmt::Display for ProcEffects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ProcEffects::*;

        match self {
            Inquire(x) => {
                write!(f, "$")?;
                x.fmt(f)
            }
            Evoke(x) => {
                write!(f, "!")?;
                x.fmt(f)
            }
        }
    }
}
