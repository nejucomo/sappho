//! TODO: Come up with a more specific name for this effect.

use sappho_unparse::{Stream, Unparse};

use crate::{Kernel, Root};

/// A [PureX] is an expression without effects
#[derive(Debug, derive_more::From)]
pub struct PureX(Kernel<PureX>);

impl Root for PureX {}

impl Unparse for PureX {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
