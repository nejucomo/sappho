use crate::{FromFx, GenExpr};
use sappho_ast as ast;
use std::fmt;

pub type ProcExpr = GenExpr<ProcEffects>;

#[derive(Clone, Debug, PartialEq)]
pub enum ProcEffects {
    Inquire(Box<GenExpr<ProcEffects>>),
    Evoke(Box<GenExpr<ProcEffects>>),
}

impl FromFx for ProcEffects {
    type AstFx = ast::ProcEffects;

    fn from_fx(astfx: ast::ProcEffects) -> Self {
        use ProcEffects::{Evoke, Inquire};

        match astfx {
            ast::ProcEffects::Inquire(x) => Inquire(Box::new(GenExpr::from(*x))),
            ast::ProcEffects::Evoke(x) => Evoke(Box::new(GenExpr::from(*x))),
        }
    }
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
