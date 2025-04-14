#[derive(Debug)]
pub enum Step<V, X, C> {
    Produce(V),
    Continue(X, C),
}
