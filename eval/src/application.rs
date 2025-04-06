use sappho_east::{Application, Expr, Wise};
use sappho_effect::Effect;
use sappho_value::{Valuable as _, Value};

use crate::continuation::{Continuation, EvalStep};
use crate::scoped::Scoped;
use crate::step::Step::{self, Continue, Produce};

use self::AppCont::*;

// BUG: Application should hold `BoxWise`; fix `-east` _and_ `-syntax`.

#[derive(Debug)]
enum AppCont<FX>
where
    FX: Effect,
{
    PendingTarget(Wise<FX>),
    PendingArg(Value),
}

impl<FX> EvalStep<FX> for Scoped<Application<FX>>
where
    FX: Effect,
{
    type Continuation = Scoped<AppCont<FX>>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation> {
        let Scoped { scope, node } = self;
        Continue(
            Scoped::new(scope.clone(), node.target),
            Some(Scoped::new(scope, PendingTarget(node.argument))),
        )
    }
}

impl<FX> Continuation<FX> for Scoped<AppCont<FX>>
where
    FX: Effect,
{
    fn eval_from_value(self, v: Value) -> Step<Scoped<Wise<FX>>, Self> {
        let Scoped { scope, node } = self;
        match node {
            PendingTarget(argexpr) => Continue(
                Scoped::new(scope.clone(), argexpr),
                Some(Scoped::new(scope, PendingArg(v))),
            ),
            PendingArg(target) => Produce(target.apply(v).unwrap()),
        }
    }
}
