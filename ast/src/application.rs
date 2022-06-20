use crate::GenExpr;
use std::fmt;

/// Function application, ie `f x`.
#[derive(Debug, PartialEq)]
pub struct ApplicationExpr<Effects> {
    /// The target of application, ie `f` in `f x`.
    pub target: Box<GenExpr<Effects>>,

    /// The argument of application, ie `x` in `f x`.
    pub argument: Box<GenExpr<Effects>>,
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
