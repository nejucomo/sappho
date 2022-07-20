use crate::{Eval, Result};
use sappho_ast_reduced::EffectExpr;
use sappho_astcore::{PureEffects, QueryEffects};
use sappho_value::{ScopeRef, ValRef};

impl Eval for EffectExpr<PureEffects> {
    fn eval(&self, _scope: &ScopeRef) -> Result<ValRef> {
        unreachable!("There are no pure effects beyond `Expr` so this should never evaluate.");
    }
}

impl Eval for EffectExpr<QueryEffects> {
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use QueryEffects::*;

        match self.effect {
            Inquire => {
                use crate::EvalThunk;
                use sappho_value::Query;

                let v = self.expr.eval(scope)?;
                let q: &Query = v.coerce()?;
                q.as_thunk().eval_thunk()
            }
        }
    }
}
