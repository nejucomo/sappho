use sappho_continuation::Continuation;
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use crate::expr::ContExpr;
use crate::exprstep::ExprStep;

pub trait EvalExpr<'scope, 'expr, FX>:
    Continuation<&'scope Scope, Value, &'expr Expr<FX>, ContExpr<'expr, FX>>
where
    FX: Effect + 'expr,
{
    fn continue_eval_with_scope(&'expr self, scope: &'scope Scope) -> ExprStep<'expr, FX>;
}

impl<'s, 'x, FX, T> Continuation<&'s Scope, Value, &'x Expr<FX>, ContExpr<'x, FX>> for T
where
    T: EvalExpr<'s, 'x, FX>,
    FX: Effect + 'x,
{
    fn continue_with(self, scope: &'s Scope) -> ExprStep<'x, FX> {
        self.continue_eval_with_scope(scope)
    }
}
