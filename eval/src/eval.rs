use sappho_east::Wise;
use sappho_effect::ProcEffect;
use sappho_scope::Scope;
use sappho_value::Value;

use crate::expr::ContExpr;
use crate::step::Step;

pub fn eval<X>(expr: X) -> Value
where
    X: Into<Wise<ProcEffect>>,
{
    let (expr, sourcecode) = expr.into().unwrap().into();
    let mut stack: Vec<ContExpr<ProcEffect>> = vec![];
    let mut step = expr.eval_step(sourcecode, Scope::default());

    loop {
        match step {
            Step::Value(v) => {
                if let Some(cont) = stack.pop() {
                    step = cont.eval_step(v);
                } else {
                    return v;
                }
            }
            Step::Continue(scope, next, cont) => {
                stack.push(cont);
                step = next.eval_step(scope);
            }
        }
    }
}
