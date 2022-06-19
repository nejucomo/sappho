use crate::{AstFxFor, FromFx, GenExpr, Pattern};
use sappho_ast as ast;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LetExpr<Effects> {
    pub binding: Pattern,
    pub bindexpr: Box<GenExpr<Effects>>,
    pub tail: Box<GenExpr<Effects>>,
}

impl<FX> fmt::Display for LetExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let ")?;
        self.binding.fmt(f)?;
        write!(f, " = ")?;
        self.bindexpr.fmt(f)?;
        write!(f, "; ")?;
        self.tail.fmt(f)?;
        Ok(())
    }
}

impl<FX> From<ast::LetExpr<AstFxFor<FX>>> for LetExpr<FX>
where
    FX: FromFx,
{
    fn from(re: ast::LetExpr<AstFxFor<FX>>) -> Self {
        LetExpr {
            binding: re.binding,
            bindexpr: Box::new(GenExpr::from(*re.bindexpr)),
            tail: Box::new(GenExpr::from(*re.tail)),
        }
    }
}
