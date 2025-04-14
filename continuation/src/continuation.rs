use crate::Step;

pub trait Continuation<V, X, C> {
    fn continue_eval(self) -> Step<V, X, C>;
}
