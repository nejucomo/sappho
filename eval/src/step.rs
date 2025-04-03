use sappho_value::Value;

use self::Step::*;

#[derive(Debug)]
pub(crate) enum Step<X, C> {
    Produce(Value),
    Continue(X, Option<C>),
}

impl<X, C> Step<X, C> {
    // pub(crate) fn map_node<F, X2>(self, f: F) -> Step<X2, C>
    // where
    //     F: FnOnce(X) -> X2,
    // {
    //     match self {
    //         Produce(v) => Produce(v),
    //         Continue(x, c) => Continue(f(x), c),
    //     }
    // }

    pub(crate) fn map_cont<F, C2>(self, f: F) -> Step<X, C2>
    where
        F: FnOnce(C) -> C2,
    {
        match self {
            Produce(v) => Produce(v),
            Continue(x, optc) => Continue(x, optc.map(f)),
        }
    }

    pub(crate) fn cont_from<C2>(self) -> Step<X, C2>
    where
        C2: From<C>,
    {
        self.map_cont(C2::from)
    }
}
