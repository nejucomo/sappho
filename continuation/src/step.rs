use derive_more::From;

use crate::withaux::WithAuxillary;
use crate::ContinueStep;

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
