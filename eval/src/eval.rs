use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use crate::scoped::ScopeExt as _;

pub fn eval<'x, FX>(expr: &'x Expr<FX>) -> Value
where
    FX: Effect,
{
    eval_in_scope(expr, Scope::default())
}

pub fn eval_in_scope<'s, 'x, FX>(expr: &'x Expr<FX>, mut scope: Scope) -> Value
where
    FX: Effect,
{
    let mut cstack = vec![];
    let mut step = scope.wrap(expr).continue_eval();

    loop {
        match step {}
    }
}
