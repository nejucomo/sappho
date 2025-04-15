/// A "Continuation Step" is an intermediate output of eval which enables an explicit heap-managed user-space stack of continuations
pub(crate) enum Step<V, X, C> {
    /// The evaluation produced a value
    Produce(V),
    /// The evaluation produced a sub-expression and continuation
    Continue(X, C),
}

impl<V, X, C> Step<V, X, C> {
    pub(crate) fn produce<W>(value: W) -> Self
    where
        W: Into<V>,
    {
        Step::Produce(value.into())
    }
}
