use crate::GenExpr;
use sappho_unparse::{Unparse, Stream};

/// Pure expressions without side-effects.
pub type PureExpr = GenExpr<PureEffects>;

/// Pure effects cannot be instantiated, because pure expressions have no side effects.
#[derive(Clone, Debug, PartialEq)]
pub enum PureEffects {}

impl Unparse for PureEffects {
    fn unparse(&self) -> Stream {
        unreachable!("pure effects are never instantiated");
    }
}
