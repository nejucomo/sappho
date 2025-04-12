use std::fmt::Debug;

pub(crate) trait EvalNext<'s, FX>: Sized + Debug {
    fn eval_next(self) -> impl Into<EvalStep<'s, FX>>;
}

pub(crate) trait Continuation<'s, FX, V>: Sized + Debug {
    fn continue_with(self, v: V) -> impl Into<EvalStep<'s, FX>>;
}
