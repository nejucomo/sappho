use derive_new::new;
use either::Either::{self, Right};
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_scope::Scope;
use sappho_source::SourceCodeRef;
use sappho_value::Value;
use sappho_with_source::WithSource;

use crate::continuation::Continuation;

pub(crate) type State = WithSource<Scope>;

pub(crate) trait EvalStep<FX>
where
    FX: Effect,
{
    type Continuation: Continuation<FX>;

    fn eval_step(self, state: State) -> Step<FX, Self::Continuation>;
}

pub(crate) type Step<FX, C> = Either<Value, EvalNext<FX, C>>;

#[derive(Debug, new)]
pub(crate) struct EvalNext<FX, C>
where
    FX: Effect,
{
    state: State,
    expr: Expr<FX>,
    cont: Option<C>,
}

impl<FX, C> EvalNext<FX, C>
where
    FX: Effect,
{
    pub(crate) fn new_ws<T>(state: State, withsrc: T, cont: Option<C>) -> Self
    where
        T: Into<(Expr<FX>, Option<SourceCodeRef>)>,
    {
        let (scope, _) = state.into();
        let (expr, sourcecode) = withsrc.into();
        Self::new(WithSource::new(scope, sourcecode), expr, cont)
    }

    pub(crate) fn wrap_continuation<C2>(self) -> Step<FX, C2>
    where
        C2: From<C>,
    {
        let EvalNext { state, expr, cont } = self;

        Right(EvalNext {
            state,
            expr,
            cont: cont.map(C2::from),
        })
    }
}
