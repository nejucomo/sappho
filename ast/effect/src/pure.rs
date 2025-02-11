use sappho_unparse::{Stream, Unparse};

use crate::Effect;

/// Pure effects cannot be instantiated, because pure expressions have no side effects.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PureEffect {}

impl Effect for PureEffect {}

impl Unparse for PureEffect {
    fn unparse_into(&self, _s: &mut Stream) {
        unreachable!("pure effects are never instantiated");
    }
}
