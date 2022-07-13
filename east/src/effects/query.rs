use crate::{FromFx, GenExpr};
use sappho_ast as ast;
use sappho_unparse::{Unparse, Stream};

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

impl Unparse for QueryEffects {
    fn unparse_into(&self, s: &mut Stream) {
        use QueryEffects::*;

        match self {
            Inquire(x) => {
                write!(f, "$")?;
                x.unparse(f, depth)
            }
        }
    }
}
