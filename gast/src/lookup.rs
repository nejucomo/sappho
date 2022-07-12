use crate::Identifier;
use sappho_unparse::{DisplayDepth, FmtResult, Formatter};

/// An attribute lookup expression, ie: `x.foo`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
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

impl<X> DisplayDepth for LookupExpr<X>
where
    X: DisplayDepth,
{
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        self.target.fmt_depth(f, depth)?;
        write!(f, ".{}", self.attr)?;
        Ok(())
    }
}
