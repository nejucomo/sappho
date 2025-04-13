use derive_more::From;

use crate::withaux::WithAuxillary;
use crate::{Continuation, ContinueStep};

use self::Step::{Continue, Produce};

#[derive(Debug, From)]
pub enum Step<V, X, C> {
    /// This continuation concluded and produced a result
    ///
    /// # TODO - `Err` type and handling
    Produce(V),
    /// Evaluate `X` and send the result to `C`
    #[from]
    Continue(ContinueStep<X, C>),
}

impl<A, V, X, C> Step<V, (A, X), C> {
    pub fn factor_auxillary(self) -> Step<V, X, WithAuxillary<C, A>> {
        match self {
            Produce(v) => Produce(v),
            Continue(cs) => Continue(cs.factor_auxillary()),
        }
    }
}

impl<V, X, C, I, A> Step<V, X, IC>
where
    I: IntoIterator<Item = (A, X)>,
    C: Continuation<(A, V), V, X, C>,
{
    pub fn from_iter(itercont: C, it: I) -> Self {
        let mut it = it.into_iter();
        if let Some(pair) = it.next() {
            ContinueStep::new(pair, IterContinuer::from(itercont))
                .with_aux(it)
                .factor_auxillary()
                .into()
        } else {
            yyy
        }
    }
}
