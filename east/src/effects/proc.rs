use crate::{FromFx, GenExpr};
use sappho_ast as ast;
use sappho_unparse::{DisplayDepth, FmtResult, Formatter};

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

impl FromFx for ast::ProcEffects {
    type AstFx = ProcEffects;

    fn from_fx(astfx: ProcEffects) -> Self {
        use ProcEffects::{Evoke, Inquire};

        match astfx {
            Inquire(x) => ast::ProcEffects::Inquire(Box::new(ast::GenExpr::from(*x))),
            Evoke(x) => ast::ProcEffects::Evoke(Box::new(ast::GenExpr::from(*x))),
        }
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
