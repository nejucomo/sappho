use std::fmt::Debug;

use crate::step::Step;

pub trait Continuation<I, O, X, N>: Sized + Debug {
    fn continue_with(self, v: I) -> Step<O, X, N>;
}
