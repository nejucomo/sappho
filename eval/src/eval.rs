use sappho_east::Wise;
use sappho_value::Value;

use crate::continuation::{Continuation, EvalStep};
use crate::evfx::{ContinuationStack as _, HasCStack};
use crate::step::Step;
use crate::EvalEffect;

pub fn eval<FX, X>(expr: X) -> Value
where
    FX: EvalEffect,
    X: Into<Wise<FX>>,
{
    eval_inner(expr.into())
}

fn eval_inner<FX>(expr: Wise<FX>) -> Value
where
    FX: HasCStack,
{
    use Step::*;

    let mut stack = FX::Stack::default();
    let mut step = expr.eval_step().cont_from();

    loop {
        match step {
            Produce(v) => {
                if let Some(cont) = stack.pop() {
                    step = cont.eval_from_value(v);
                } else {
                    return v;
                }
            }
            Continue(next, optcont) => {
                if let Some(cont) = optcont {
                    stack.push(cont);
                }
                step = next.eval_step().cont_from();
            }
        }
    }
}
