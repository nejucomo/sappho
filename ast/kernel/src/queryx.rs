use sappho_unparse::{Stream, Unparse};

use crate::{Confined, Kernel};

/// A [QueryX] is an expression with query effects
#[derive(Debug, derive_more::From)]
pub enum QueryX<U>
where
    U: Unparse,
{
    Kernel(Kernel<U>),
    // Syntax mayber-bug: do we want to allow `$$x` rather than requiring `$($x)`?
    // If so, then `QueryX` would need to extend `Confined` which I think requires passing two params everywhere (then three for proc?)
    Inquire(Confined<U>),
}

impl<U> Unparse for QueryX<U>
where
    U: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use QueryX::*;

        match self {
            Kernel(x) => x.unparse_into(s),
            Inquire(x) => x.unparse_into(s),
        }
    }
}
