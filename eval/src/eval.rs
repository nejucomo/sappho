use sappho_effect::Effect;
use sappho_value::Value;

use crate::continuation::EvalNext;
use crate::step::EvalStep::{self, *};

pub fn eval<'s, FX>(expr: &'s Expr<FX>) -> Value
where
    FX: Effect,
{
    eval_in_scope(expr, &Scope::default())
}

pub fn eval_in_scope<'s, FX>(expr: &'s Expr<FX>, scope: &Scope) -> Value
where
    FX: Effect,
{
    let mut continuations = vec![];
    let mut step: EvalStep<'s, FX> = expr.eval_next().into();

    loop {
        match step {
            Produce(v) => {
                if let Some(cont) = continuations.pop() {
                    step = cont.continue_with(v).into();
                } else {
                    return v;
                }
            }
            Continue(x, c) => {
                continuations.push(c);
                step = x.eval_next().into();
            }
        }
    }
}
