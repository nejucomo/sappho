use crate::{GenExpr, Identifier};
use std::fmt;

/// An attribute lookup expression, ie: `x.foo`.
#[derive(Debug, PartialEq)]
pub struct LookupExpr<Effects> {
    /// The target expression of the lookup, ie `x` in `x.foo`.
    pub target: Box<GenExpr<Effects>>,

    /// An attribute name, ie: `foo` in `x.foo`.
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
