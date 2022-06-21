use super::Eval;
use crate::Result;
use sappho_east::{PureEffects, QueryEffects};
use sappho_value::{ScopeRef, ValRef};

impl Eval for PureEffects {
    fn eval(&self, _scope: &ScopeRef) -> Result<ValRef> {
        unreachable!("There are no pure effects beyond `GenExpr` so theis should never evaluate.");
    }
}

impl Eval for QueryEffects {
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        match self {
            QueryEffects::Inquire(qexpr) => qexpr.eval(scope),
        }
    }
}
