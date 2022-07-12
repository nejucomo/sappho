use crate::GenExpr;
use sappho_unparse::{DisplayDepth, FmtResult, Formatter};

/// Pure expressions without side-effects.
pub type PureExpr = GenExpr<PureEffects>;

/// Pure effects cannot be instantiated, because pure expressions have no side effects.
#[derive(Clone, Debug, PartialEq)]
pub enum PureEffects {}

impl DisplayDepth for PureEffects {
    fn fmt_depth(&self, _f: &mut Formatter, _depth: usize) -> FmtResult {
        unreachable!("pure effects are never instantiated");
    }
}
