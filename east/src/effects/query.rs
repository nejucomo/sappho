use crate::{Expr, FromFx};
use sappho_ast as ast;
use sappho_unparse::{Stream, Unparse};

pub type QueryExpr = Expr<QueryEffects>;

#[derive(Clone, Debug, PartialEq)]
pub enum QueryEffects {
    Inquire(Box<Expr<QueryEffects>>),
}

impl FromFx for QueryEffects {
    type AstFx = ast::QueryEffects;

    fn from_fx(astfx: ast::QueryEffects) -> Self {
        use QueryEffects::Inquire;

        match astfx {
            ast::QueryEffects::Inquire(x) => Inquire(Box::new(Expr::from(*x))),
        }
    }
}

impl FromFx for ast::QueryEffects {
    type AstFx = QueryEffects;

    fn from_fx(astfx: QueryEffects) -> Self {
        use QueryEffects::Inquire;

        match astfx {
            Inquire(x) => ast::QueryEffects::Inquire(Box::new(ast::Expr::from(*x))),
        }
    }
}

impl Unparse for QueryEffects {
    fn unparse_into(&self, s: &mut Stream) {
        use QueryEffects::*;

        match self {
            Inquire(x) => {
                s.write("$");
                s.write(x);
            }
        }
    }
}
