use crate::{Eval, Result};
use sappho_ast_core::{PureEffect, QueryEffect};
use sappho_ast_reduced::EffectExpr;
use sappho_value::{ScopeRef, ValRef};

impl Eval for EffectExpr<PureEffect> {
    fn eval(&self, _scope: &ScopeRef) -> Result<ValRef> {
        unreachable!("There are no pure effects beyond `Expr` so this should never evaluate.");
    }
}

impl Eval for EffectExpr<QueryEffect> {
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use QueryEffect::*;

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
