use crate::{Eval, Result};
use sappho_east::ApplicationExpr;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for ApplicationExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use sappho_value::Func;

        let ApplicationExpr { target, argument } = self;
        let tval = target.eval(scope)?;
        let aval = argument.eval(scope)?;
        let func: &Func = tval.coerce()?;
        let (expr, boundscope) = func.bind_arg(&aval);
        expr.eval(&boundscope)
    }
}
