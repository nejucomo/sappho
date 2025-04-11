use std::fmt::Debug;

use crate::{Eval, Scope, Step};

pub trait EvalStep<'s, X>: Sized + Debug
where
    X: Eval<'s>,
{
    type Continuation: Continuation<'s, X>;

    fn eval_step(&'s self, scope: &Scope<X>) -> Step<'s, X, Self::Continuation>;
}

pub trait Continuation<'s, X>: Sized + Debug
where
    X: Eval<'s>,
{
    type Auxillary: Debug;

    fn continue_with_value(self, value: X::Value, aux: Self::Auxillary) -> Step<'s, X, Self>;
}
