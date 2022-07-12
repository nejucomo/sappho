use crate::GenExpr;
use sappho_unparse::{DisplayDepth, FmtResult, Formatter};

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

impl From<ProcEffects> for ProcExpr {
    fn from(x: ProcEffects) -> Self {
        GenExpr::Effect(x)
    }
}

impl DisplayDepth for ProcEffects {
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        use ProcEffects::*;

        match self {
            Inquire(x) => {
                write!(f, "$")?;
                x.fmt_depth(f, depth)
            }
            Evoke(x) => {
                write!(f, "!")?;
                x.fmt_depth(f, depth)
            }
        }
    }
}
