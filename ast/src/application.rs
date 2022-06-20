use crate::GenExpr;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ApplicationExpr<Effects> {
    pub target: Box<GenExpr<Effects>>,
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
