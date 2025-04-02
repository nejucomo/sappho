use derive_more::From;
use sappho_primval::PrimVal;
use sappho_value::Value;

#[derive(Debug, From)]
pub(crate) enum Step<N, C> {
    #[from(Value, PrimVal)]
    Value(Value),
    /// Continue with `N` (next) feeding the resulting value to `C` (continuation)
    Continue(N, C),
}

impl<N, C> Step<N, C> {
    pub(crate) fn map_continuation<F, C2>(self, f: F) -> Step<N, C2>
    where
        F: FnOnce(C) -> C2,
    {
        use Step::*;

        match self {
            Value(v) => Value(v),
            Continue(n, c) => Continue(n, f(c)),
        }
    }
}
