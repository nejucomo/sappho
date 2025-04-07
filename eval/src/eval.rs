use sappho_east::Wise;
use sappho_effect::Effect;
use sappho_value::Value;

use crate::continuation::{Continuation, EvalStep};
use crate::expr::ExprCont;
use crate::step::Step;

pub fn eval<FX, X>(expr: X) -> Value
where
    FX: Effect,
    X: Into<Wise<FX>>,
{
    use Step::*;

    let mut stack: Vec<ExprCont<FX>> = vec![];
    let mut step = expr.into().eval_step();

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
                step = next.eval_step();
            }
        }
    }
}
