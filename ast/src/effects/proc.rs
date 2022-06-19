use crate::GenExpr;
use std::fmt;

pub type ProcExpr = GenExpr<ProcEffects>;

#[derive(Debug, PartialEq)]
pub enum ProcEffects {
    Inquire(Box<GenExpr<ProcEffects>>),
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
