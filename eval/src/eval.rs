use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use crate::exprstep::ExprStep;

pub fn eval<'a, FX>(expr: &'a Expr<FX>) -> Value
where
    FX: Effect,
{
    eval_in_scope(expr, Scope::default())
}

pub fn eval_in_scope<'a, FX>(expr: &'a Expr<FX>, mut scope: Scope) -> Value
where
    FX: Effect,
{
    let mut continuations = vec![];
    let mut step: ExprStep<'a, FX> = expr.eval(&scope);

    loop {
        use crate::step::Step::{Continue, Produce};

        match step {
            Produce(v) => {
                if let Some(c) = continuations.pop() {
                    step = c.eval(v);
                } else {
                    return v;
                }
            }

            Continue(x, c) => {
                continuations.push(c);

                step = x.eval(&scope);
            }
        }
    }
}
