use std::fmt::Debug;

pub trait EvalStep<V, X, C>: Sized + Debug {
    fn eval_step(self) -> Step<V, X, C>;
}

pub trait Continuation<V, X>: Sized + Debug {
    fn continue_with_value(self, value: V) -> Step<V, X, Self>;
}

#[derive(Debug)]
pub enum Step<V, X, C> {
    /// This continuation concluded and produced a value
    Conclude(V),
    /// Evaluate `X` and send the result to `C`
    Continue(X, C),
}
