use crate::{Eval, Result};
use sappho_east::{PureEffects, QueryEffects};
use sappho_value::{ScopeRef, ValRef};

impl Eval for PureEffects {
    fn eval(&self, _scope: &ScopeRef) -> Result<ValRef> {
        unreachable!("There are no pure effects beyond `Expr` so theis should never evaluate.");
    }
}

impl Eval for QueryEffects {
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        match self {
            QueryEffects::Inquire(qexpr) => {
                use crate::EvalThunk;
                use sappho_value::Query;

                let v = qexpr.eval(scope)?;
                let q: &Query = v.coerce()?;
                q.as_thunk().eval_thunk()
            }
        }
    }
}
