use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use crate::step::Step;

// BUG: These should carry through a `Scope` and `Scoped` should be removed.

pub(crate) trait EvalStep<FX>: Sized
where
    FX: Effect,
{
    type Continuation<'a>: Continuation<'a, FX>
    where
        Self: 'a;

    fn eval_step<'a>(&'a self, scope: &Scope) -> Step<'a, FX, Self::Continuation<'a>>;
}

pub(crate) trait Continuation<'a, FX>: Sized
where
    FX: Effect,
{
    fn eval_from_value(self, scope: Scope, v: Value) -> Step<'a, FX, Self>;
}
