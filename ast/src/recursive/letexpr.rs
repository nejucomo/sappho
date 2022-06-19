use crate::{GenExpr, Pattern};
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
