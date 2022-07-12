use crate::{FromFx, GenExpr};
use sappho_ast as ast;
use sappho_unparse::{DisplayDepth, FmtResult, Formatter};

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
