use crate::GenExpr;
use std::fmt;

/// Pure expressions without side-effects.
pub type PureExpr = GenExpr<PureEffects>;

/// Pure effects cannot be instantiated, because pure expressions have no side effects.
#[derive(Clone, Debug, PartialEq)]
pub enum PureEffects {}

impl fmt::Display for PureEffects {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unreachable!("pure effects are never instantiated");
    }
}
