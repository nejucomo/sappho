//! TODO: Come up with a more specific name for this effect.

use sappho_unparse::{Stream, Unparse};

use crate::Kernel;

/// A [PureX] is an expression without effects
#[derive(Debug, derive_more::From)]
pub struct PureX<U>(Kernel<U>)
where
    U: Unparse + From<PureX<U>>;

impl<U> Unparse for PureX<U>
where
    U: Unparse + From<PureX<U>>,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
