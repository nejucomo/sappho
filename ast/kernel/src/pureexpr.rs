//! TODO: Come up with a more specific name for this effect.

use sappho_unparse::{Stream, Unparse};

use crate::Kernel;

/// An expression without effects
#[derive(Debug, derive_more::From)]
pub struct PureExpr<U>(Kernel<U>)
where
    U: Unparse;

impl<U> Unparse for PureExpr<U>
where
    U: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
