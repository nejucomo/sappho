use sappho_effect::Effect;
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct ItcState<'s, FX, IC>
where
    FX: Effect,
    IC: IterContinuation<'s, FX>,
{
    builder: IC,
    iter: IC::Iter,
    aux: IC::Aux,
}

pub(crate) trait IterContinuation<'s, FX>
where
    FX: Effect,
{
    type Iter: Iterator;
    type Aux: Debug;

    fn eval_next_from_iter(self, iter: Self::Iter) -> EvalStep<'s, FX>;
}
