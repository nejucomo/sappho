use derive_more::From;
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_scope::{Scope, Scoped};
use sappho_value::Value;

use crate::continuation::Continuation;
use crate::letexpr::ContLet;
use crate::step::{EvalStep, State, Step};

impl<FX> EvalStep<FX> for Expr<FX>
where
    FX: Effect,
{
    type Continuation = ContExpr<FX>;

    fn eval_step(self, state: State) -> Step<FX, Self::Continuation> {
        use Expr::*;

        match self {
            Prim(x) => x.into(),
            Let(x) => x.eval(state).wrap_continuation(),
        }
    }
}

#[derive(Debug, From)]
pub(crate) enum ContExpr<FX>
where
    FX: Effect,
{
    Let(ContLet<FX>),
}

impl<FX> Continuation<FX> for ContExpr<FX>
where
    FX: Effect,
{
    fn continue_eval(self, v: Value) -> Step<FX, Self> {
        use ContExpr::*;

        match node {
            Let(x) => x.continue_eval(v),
        }
    }
}
