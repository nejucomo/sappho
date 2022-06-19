mod apply;
mod letexpr;

use crate::{AstFxFor, FromFx, GenExpr};
use sappho_ast as ast;
use std::fmt;

pub use self::apply::Application;
pub use self::letexpr::LetExpr;

#[derive(Debug, PartialEq)]
pub enum RecursiveExpr<Effects> {
    List(Vec<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Apply(Application<Effects>),
}

impl<FX> From<ast::RecursiveExpr<AstFxFor<FX>>> for RecursiveExpr<FX>
where
    FX: FromFx,
{
    fn from(re: ast::RecursiveExpr<AstFxFor<FX>>) -> Self {
        use RecursiveExpr::*;

        match re {
            ast::RecursiveExpr::List(x) => List(x.into_iter().map(GenExpr::from).collect()),
            ast::RecursiveExpr::Let(x) => Let(LetExpr::from(x)),
            ast::RecursiveExpr::Apply(x) => Apply(Application::from(x)),
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
        }
    }
}
