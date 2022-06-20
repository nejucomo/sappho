use crate::{GenExpr, Identifier};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LookupExpr<Effects> {
    pub target: Box<GenExpr<Effects>>,
    pub attr: Identifier,
}

impl<FX> fmt::Display for LookupExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.target.fmt(f)?;
        write!(f, ".{}", self.attr)?;
        Ok(())
    }
}
