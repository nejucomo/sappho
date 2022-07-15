use crate::Expr;
use sappho_unparse::{Stream, Unparse};

/// Pure expressions without side-effects.
pub type PureExpr = Expr<PureEffects>;

/// Pure effects cannot be instantiated, because pure expressions have no side effects.
#[derive(Clone, Debug, PartialEq)]
pub enum PureEffects {}

impl Unparse for PureEffects {
    fn unparse_into(&self, _s: &mut Stream) {
        unreachable!("pure effects are never instantiated");
    }
}
