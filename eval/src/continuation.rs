use sappho_value::Value;

use crate::step::Step;

pub(crate) trait Continuation: Sized {
    type Next;

    fn eval_step(self, v: Value) -> Step<Self, Self::Next>;
}
