use crate::{GenExpr, Pattern};
use std::fmt;

/// A `let` expression for local definitions, ie: `let x = 42; f x`.
#[derive(Debug, PartialEq)]
pub struct LetExpr<Effects> {
    /// The binding pattern, ie: the first `x` in `let x = 42; f x`.
    pub binding: Pattern,

    /// The expression to bind, ie: `42` in `let x = 42; f x`.
    pub bindexpr: Box<GenExpr<Effects>>,

    /// The expression to evaluate with the binding in-scope, ie: `f x` in `let x = 42; f x`.
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
