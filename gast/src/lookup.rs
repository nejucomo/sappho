use crate::Identifier;
use std::fmt;

/// An attribute lookup expression, ie: `x.foo`.
#[derive(Debug, PartialEq, derive_new::new)]
pub struct LookupExpr<Expr> {
    /// The target expression of the lookup, ie `x` in `x.foo`.
    pub target: Box<Expr>,

    /// An attribute name, ie: `foo` in `x.foo`.
    pub attr: Identifier,
}

impl<X> LookupExpr<X> {
    pub fn transform_into<Y>(self) -> LookupExpr<Y>
    where
        Y: From<X>,
    {
        LookupExpr {
            target: Box::new(Y::from(*self.target)),
            attr: self.attr,
        }
    }
}

impl<X> fmt::Display for LookupExpr<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.target.fmt(f)?;
        write!(f, ".{}", self.attr)?;
        Ok(())
    }
}
