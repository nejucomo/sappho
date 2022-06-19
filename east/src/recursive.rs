mod apply;
mod letexpr;
mod lookup;

use crate::{AstFxFor, FromFx, GenExpr};
use sappho_ast as ast;
use std::fmt;

pub use self::apply::Application;
pub use self::letexpr::LetExpr;
pub use self::lookup::Lookup;

#[derive(Debug, PartialEq)]
pub enum RecursiveExpr<Effects> {
    List(Vec<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Apply(Application<Effects>),
    Lookup(Lookup<Effects>),
}

impl<FX> From<ast::RecursiveExpr<AstFxFor<FX>>> for RecursiveExpr<FX>
where
    FX: FromFx,
{
    fn from(re: ast::RecursiveExpr<AstFxFor<FX>>) -> Self {
        use ast::RecursiveExpr as ARE;
        use RecursiveExpr as ERE;

        match re {
            ARE::List(x) => ERE::List(x.into_iter().map(GenExpr::from).collect()),
            ARE::Let(x) => ERE::Let(LetExpr::from(x)),
            ARE::Apply(x) => ERE::Apply(Application::from(x)),
            ARE::Lookup(x) => ERE::Lookup(Lookup::from(x)),
        }
    }
}

impl<FX> fmt::Display for RecursiveExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RecursiveExpr::*;

        match self {
            List(x) => {
                let mut first = true;
                write!(f, "[")?;
                for child in x.iter() {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    child.fmt(f)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            Let(x) => x.fmt(f),
            Apply(x) => x.fmt(f),
            Lookup(x) => x.fmt(f),
        }
    }
}
