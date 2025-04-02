use sappho_effect::Effect;
use sappho_value::Value;

use crate::step::EvalStep;

pub(crate) trait Continuation<FX>: Sized
where
    FX: Effect,
{
    type EvalStep: EvalStep<FX>;

    fn continue_eval(self, v: Value) -> Self::EvalStep;
}
