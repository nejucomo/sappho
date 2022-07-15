use crate::{Expr, FromFx};
use sappho_ast as ast;
use sappho_unparse::{Stream, Unparse};

pub type ProcExpr = Expr<ProcEffects>;

#[derive(Clone, Debug, PartialEq)]
pub enum ProcEffects {
    Inquire(Box<Expr<ProcEffects>>),
    Evoke(Box<Expr<ProcEffects>>),
}

impl FromFx for ProcEffects {
    type AstFx = ast::ProcEffects;

    fn from_fx(astfx: ast::ProcEffects) -> Self {
        use ProcEffects::{Evoke, Inquire};

        match astfx {
            ast::ProcEffects::Inquire(x) => Inquire(Box::new(Expr::from(*x))),
            ast::ProcEffects::Evoke(x) => Evoke(Box::new(Expr::from(*x))),
        }
    }
}

impl FromFx for ast::ProcEffects {
    type AstFx = ProcEffects;

    fn from_fx(astfx: ProcEffects) -> Self {
        use ProcEffects::{Evoke, Inquire};

        match astfx {
            Inquire(x) => ast::ProcEffects::Inquire(Box::new(ast::Expr::from(*x))),
            Evoke(x) => ast::ProcEffects::Evoke(Box::new(ast::Expr::from(*x))),
        }
    }
}

impl Unparse for ProcEffects {
    fn unparse_into(&self, s: &mut Stream) {
        use ProcEffects::*;

        match self {
            Inquire(x) => {
                s.write("$");
                s.write(x);
            }
            Evoke(x) => {
                s.write("!");
                s.write(x);
            }
        }
    }
}
