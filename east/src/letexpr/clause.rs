use crate::{AstFxFor, FromFx, GenExpr, Pattern};
use sappho_ast as ast;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LetClause<Effects> {
    /// The binding pattern, ie: the first `x` in `let x = 42; f x`.
    pub binding: Pattern,

    /// The expression to bind, ie: `42` in `let x = 42; f x`.
    pub bindexpr: Box<GenExpr<Effects>>,
}

impl<FX> fmt::Display for LetClause<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let ")?;
        self.binding.fmt(f)?;
        write!(f, " = ")?;
        self.bindexpr.fmt(f)?;
        Ok(())
    }
}

impl<FX> From<ast::LetClause<AstFxFor<FX>>> for LetClause<FX>
where
    FX: FromFx,
{
    fn from(lc: ast::LetClause<AstFxFor<FX>>) -> Self {
        LetClause {
            binding: lc.binding,
            bindexpr: Box::new(GenExpr::from(*lc.bindexpr)),
        }
    }
}
