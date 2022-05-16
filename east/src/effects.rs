use crate::GenExpr;

use saplang_ast as ast;

pub use saplang_ast::PureEffects;
pub type PureExpr = GenExpr<PureEffects>;
pub type QueryExpr = GenExpr<QueryEffects>;
pub type ProcExpr = GenExpr<ProcEffects>;

#[derive(Debug, PartialEq)]
pub enum QueryEffects {
    Inquire(Box<GenExpr<QueryEffects>>),
}

#[derive(Debug, PartialEq)]
pub enum ProcEffects {
    Inquire(Box<GenExpr<ProcEffects>>),
    Evoke(Box<GenExpr<ProcEffects>>),
}

pub trait FromFx {
    type AstFx;

    fn from_fx(astfx: Self::AstFx) -> Self;
}

pub type AstFxFor<FX> = <FX as FromFx>::AstFx;

impl FromFx for PureEffects {
    type AstFx = Self;

    fn from_fx(astfx: Self) -> Self {
        astfx
    }
}

impl FromFx for QueryEffects {
    type AstFx = ast::QueryEffects;

    fn from_fx(astfx: ast::QueryEffects) -> Self {
        use QueryEffects::Inquire;

        match astfx {
            ast::QueryEffects::Inquire(x) => Inquire(Box::new(GenExpr::from(*x))),
        }
    }
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
