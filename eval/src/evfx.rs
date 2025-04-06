//! The set of `eval`-able effects
//!
//! This module-subtree is spaghetti so that we have a concise legible public interface while also hanging private functionality off of the [EvalEffect] types via `HasCStack`
use sappho_east::Wise;
use sappho_effect::{Effect, PureEffect};

use crate::continuation::{Continuation, EvalStep};
use crate::expr::ExprCont;

use self::sealed::Sealed;

/// [Effect]s for which we can call [eval](crate::eval) on an `Expr<Self>`
pub trait EvalEffect: Effect + Sealed {}

impl EvalEffect for PureEffect {}
// impl EvalEffect for QueryEffect {}
// impl EvalEffect for ProcEffect {}

mod sealed {
    #[allow(private_bounds)]
    pub trait Sealed: super::HasCStack {}

    impl<T> Sealed for T where T: super::HasCStack {}
}

pub(crate) trait HasCStack: Effect {
    type MultiExpr: EvalStep<Self> + From<Wise<Self>>;
    type MultiCont: Continuation<Self> + From<ExprCont<Self>>;
    type Stack: ContinuationStack<Self>;
}

pub(crate) trait ContinuationStack<FX>: Default
where
    FX: HasCStack,
{
    fn push(&mut self, txc: FX::MultiCont);
    fn pop(&mut self) -> Option<FX::MultiCont>;
}
