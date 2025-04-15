use sappho_east::Wise;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use crate::contexpr::ContExpr;
use crate::evaltrait::Eval as _;
use crate::exprstep::ExprStep;

pub fn eval<FX>(expr: &Wise<FX>) -> Value
where
    FX: Effect,
{
    eval_in_scope(expr, Scope::default())
}

pub fn eval_in_scope<'a, FX>(expr: &'a Wise<FX>, scope: Scope) -> Value
where
    FX: Effect,
{
    let mut continuations: Vec<ContExpr<'a, FX>> = vec![];
    let mut step: ExprStep<'a, FX> = expr.eval(&scope);

    loop {
        use crate::step::Step::{ContinueVia, Produce};

        match step {
            Produce(v) => {
                if let Some(c) = continuations.pop() {
                    step = c.eval(v);
                } else {
                    return v;
                }
            }

            ContinueVia(x, c) => {
                continuations.push(c);

                step = x.eval(&scope);
            }
        }
    }
}
