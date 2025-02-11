use crate::{Eval, Result};
use sappho_ast::Effect;
use sappho_ast_core::{ApplicationExpr, EffectExpr};
use sappho_ast_reduced::AstRed;
use sappho_value::{ScopeRef, ValRef};

impl<FX> Eval for ApplicationExpr<AstRed, FX>
where
    EffectExpr<AstRed, FX>: Eval,
    FX: Effect,
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
