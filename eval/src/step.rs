use crate::{Continuation, Eval, StepContinue};

use self::Step::*;

#[derive(Debug)]
pub enum Step<'s, X, C>
where
    X: Eval<'s>,
    C: Continuation<'s, X>,
{
    /// This continuation concluded and produced a [Value]
    Conclude(X::Value),
    /// This continuation requires evaluating a subexpression to continue
    Continue(StepContinue<'s, X, C>),
}

impl<'s, X, C> Step<'s, X, C>
where
    X: Eval<'s>,
    C: Continuation<'s, X>,
{
    pub fn map<F, C2, G>(self, map_cont: F, map_aux: G) -> Step<'s, X, C2>
    where
        C2: Continuation<'s, X>,
        F: FnOnce(C) -> C2,
        G: FnOnce(C::Auxillary) -> C2::Auxillary,
    {
        match self {
            Conclude(v) => Conclude(v),
            Continue(cs) => Continue(cs.map(map_cont, map_aux)),
        }
    }

    pub fn cont_from<C2>(self) -> Step<'s, X, C2>
    where
        C2: From<C> + Continuation<'s, X>,
        C2::Auxillary: From<C::Auxillary>,
    {
        self.map(C2::from, C2::Auxillary::from)
    }
}
