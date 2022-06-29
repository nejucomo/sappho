use crate::GenExpr;
use sappho_gast::Pattern;
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
