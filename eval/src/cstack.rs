use crate::{Continuation, Eval};

#[derive(Debug)]
pub(crate) struct ContinuationStack<'s, X>(Vec<CStackItem<'s, X>>)
where
    X: Eval<'s>;

impl<'s, X> Default for ContinuationStack<'s, X>
where
    X: Eval<'s>,
{
    fn default() -> Self {
        ContinuationStack(vec![])
    }
}

impl<'s, X> ContinuationStack<'s, X>
where
    X: Eval<'s>,
{
    pub(crate) fn push_continuation(
        &mut self,
        cont: X::Continuation,
        aux: <X::Continuation as Continuation<'s, X>>::Auxillary,
    ) {
        self.0.push(CStackItem { cont, aux });
    }

    pub(crate) fn pop_continuation(
        &mut self,
    ) -> Option<(
        X::Continuation,
        <X::Continuation as Continuation<'s, X>>::Auxillary,
    )> {
        self.0.pop().map(|CStackItem { cont, aux }| (cont, aux))
    }
}

#[derive(Debug)]
struct CStackItem<'s, X>
where
    X: Eval<'s>,
{
    cont: X::Continuation,
    aux: <X::Continuation as Continuation<'s, X>>::Auxillary,
}
