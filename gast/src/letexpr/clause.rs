use crate::Pattern;
use std::fmt;

#[derive(Debug, PartialEq, derive_new::new)]
pub struct LetClause<Expr> {
    /// The binding pattern, ie: the first `x` in `let x = 42; f x`.
    pub binding: Pattern,

    /// The expression to bind, ie: `42` in `let x = 42; f x`.
    pub bindexpr: Box<Expr>,
}

impl<X> fmt::Display for LetClause<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let ")?;
        self.binding.fmt(f)?;
        write!(f, " = ")?;
        self.bindexpr.fmt(f)?;
        Ok(())
    }
}
