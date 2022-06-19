use crate::{GenExpr, Identifier};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Lookup<Effects> {
    pub target: Box<GenExpr<Effects>>,
    pub field: Identifier,
}

impl<FX> fmt::Display for Lookup<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        self.target.fmt(f)?;
        write!(f, ").{}", self.field)?;
        Ok(())
    }
}
