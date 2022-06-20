use crate::{AstFxFor, FromFx, GenExpr};
use sappho_ast as ast;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ApplicationExpr<Effects> {
    pub target: Box<GenExpr<Effects>>,
    pub argument: Box<GenExpr<Effects>>,
}

impl<FX> From<ast::ApplicationExpr<AstFxFor<FX>>> for ApplicationExpr<FX>
where
    FX: FromFx,
{
    fn from(app: ast::ApplicationExpr<AstFxFor<FX>>) -> Self {
        ApplicationExpr {
            target: Box::new(GenExpr::from(*app.target)),
            argument: Box::new(GenExpr::from(*app.argument)),
        }
    }
}

impl<FX> fmt::Display for ApplicationExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        self.target.fmt(f)?;
        write!(f, " ")?;
        self.argument.fmt(f)?;
        write!(f, ")")?;
        Ok(())
    }
}
