use derive_new::new;
use std::fmt::Debug;

use crate::continuation::Continuation;
use crate::step::Step;

#[derive(Debug, new)]
pub struct WithAuxillary<C, A> {
    inner: C,
    aux: A,
}

impl<A, V, X, C> Continuation<V, V, X, Self> for WithAuxillary<C, A>
where
    A: Debug,
    C: Continuation<(A, V), V, (A, X), C>,
{
    fn continue_with(self, v: V) -> Step<V, X, Self> {
        self.inner.continue_with((self.aux, v)).factor_auxillary()
    }
}
