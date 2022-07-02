use crate::{FromFx, GenExpr};
use sappho_ast as ast;
use std::fmt;

pub type QueryExpr = GenExpr<QueryEffects>;

#[derive(Clone, Debug, PartialEq)]
pub enum QueryEffects {
    Inquire(Box<GenExpr<QueryEffects>>),
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

impl FromFx for ast::QueryEffects {
    type AstFx = QueryEffects;

    fn from_fx(astfx: QueryEffects) -> Self {
        use QueryEffects::Inquire;

        match astfx {
            Inquire(x) => ast::QueryEffects::Inquire(Box::new(ast::GenExpr::from(*x))),
        }
    }
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
