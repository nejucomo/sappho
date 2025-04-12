use enum_dispatch::enum_dispatch;
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use crate::continuation::EvalNext;
use crate::evalstep::EvalStep;
use crate::step::ContinueStep;
use crate::withlocals::WithLocals;

#[enum_dispatch]
pub(crate) trait ExprEvalNext<'s, FX>:
    EvalNext<Value, WithLocals<&'s Expr<FX>>, ContExpr<'s, FX>>
where
    FX: Effect + 's,
{
    fn expr_eval_next(self, scope: &Scope) -> EvalStep<'s, FX>;
}

impl<'s, FX, T> EvalNext<Value, WithLocals<&'s Expr<FX>>, ContExpr<'s, FX>> for T
where
    T: ExprEvalNext<'s, FX>,
    FX: Effect + 's,
{
    fn eval_next(self, scope: &Scope) -> EvalStep<'s, FX> {
        self.expr_eval_next(scope)
    }
}

impl<'s, FX> ExprEvalNext<'s, FX> for &'s Expr<FX>
where
    FX: Effect,
{
    fn expr_eval_next(self, scope: &Scope) -> EvalStep<'s, FX> {
        use Expr::*;

        match self {
            Prim(x) => x.into(),
            Ref(x) => scope.lookup(x).into(),
            ObjectDef(x) => x.eval_step(scope),
            ListDef(x) => x.eval_step(scope),
            Let(x) => x.eval_step(scope),
            Match(x) => x.eval_step(scope),
            Application(x) => x.eval_step(scope),
            Lookup(x) => x.eval_step(scope),
            Interaction(x) => x.eval_step(scope),
        }
    }
}

#[derive(Debug)]
#[enum_dispatch(ExprEvalNext)]
pub(crate) enum ContExpr<'s, FX> {
    ObjectDef(ContObjectDef<'s, FX>),
}

impl<'s, FX> ContExpr<'s, FX>
where
    FX: Effect,
{
    pub(crate) fn continue_from_expr(self, expr: &'s Expr<FX>) -> EvalStep<'s, FX> {
        ContinueStep::new(expr, self).into()
    }
}
