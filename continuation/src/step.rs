#[derive(Debug)]
pub enum Step<V, X, C> {
    Produce(V),
    Continue(X, C),
}

impl<V, X, C> Step<V, X, C> {
    pub fn produce<T>(value: T) -> Self
    where
        T: Into<V>,
    {
        Step::Produce(value.into())
    }
}
