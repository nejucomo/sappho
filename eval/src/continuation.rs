mod kastimpls;

use sappho_east::Wise;
use sappho_effect::Effect;
use sappho_value::Value;

use crate::scoped::Scoped;
use crate::step::Step;

pub(crate) trait EvalStep<FX>: Sized
where
    FX: Effect,
{
    type Continuation: Continuation<FX>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation>;
}

pub(crate) trait Continuation<FX>: Sized
where
    FX: Effect,
{
    fn eval_from_value(self, v: Value) -> Step<Scoped<Wise<FX>>, Self>;
}
