use sappho_east::Wise;
use sappho_effect::PureEffect;
use sappho_value::Value;

use crate::continuation::{Continuation, EvalStep};
use crate::expr::ExprCont;
use crate::step::Step;

pub fn eval<X>(expr: X) -> Value
where
    X: Into<Wise<PureEffect>>,
{
    use Step::*;

    let mut stack: Vec<ExprCont<PureEffect>> = vec![];
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
