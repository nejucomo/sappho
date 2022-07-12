use crate::{Eval, Result};
use sappho_east::ApplicationExpr;
use sappho_fmtutil::DisplayDepth;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for ApplicationExpr<FX>
where
    FX: Eval + DisplayDepth + DisplayDepth,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use crate::EvalThunk;
        use sappho_value::Func;

        let ApplicationExpr { target, argument } = self;
        let tval = target.eval(scope)?;
        let aval = argument.eval(scope)?;
        let func: &Func = tval.coerce()?;
        let thunk = func.bind_arg(&aval)?;
        thunk.eval_thunk()
    }
}
