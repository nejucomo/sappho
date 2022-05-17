use super::Eval;
use crate::scope::ScopeRef;
use crate::{Result, ValRef};
use saplang_east::PureEffects;

impl Eval for PureEffects {
    fn eval(&self, _scope: ScopeRef) -> Result<ValRef> {
        unreachable!("There are no pure effects beyond `GenExpr` so theis should never evaluate.");
    }
}
