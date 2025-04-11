use crate::{Continuation, Eval};

#[derive(Debug)]
pub struct StepContinue<'s, X, C>
where
    X: Eval<'s>,
    C: Continuation<'s, X>,
{
    /// The continuation
    pub continuation: C,
    /// The next expression to evaluate for this continuation
    pub next_expr: &'s X,
    /// Auxillary data to associate with the [Self::wise] and the resulting [Value] computed from it
    pub aux: C::Auxillary,
    /// Push the given scope before continuing
    ///
    /// If present, that scope will be popped upon the conclusion of [Self::wise] evaluation
    pub locals: Option<X::Locals>,
}

impl<'s, X, C> StepContinue<'s, X, C>
where
    X: Eval<'s>,
    C: Continuation<'s, X>,
{
    pub fn map<F, C2, G>(self, map_cont: F, map_aux: G) -> StepContinue<'s, X, C2>
    where
        C2: Continuation<'s, X>,
        F: FnOnce(C) -> C2,
        G: FnOnce(C::Auxillary) -> C2::Auxillary,
    {
        StepContinue {
            continuation: map_cont(self.continuation),
            next_expr: self.next_expr,
            aux: map_aux(self.aux),
            locals: self.locals,
        }
    }
}
