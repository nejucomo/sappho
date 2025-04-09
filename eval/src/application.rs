use sappho_east::{Application, BoxWise, Expr, Wise};
use sappho_effect::Effect;
use sappho_value::Value;

use crate::continuation::{Continuation, EvalStep};
use crate::scoped::Scoped;
use crate::step::Step::{self, Continue, Produce};

use self::AppCont::*;

#[derive(Debug)]
enum AppCont<FX>
where
    FX: Effect,
{
    PendingTarget(BoxWise<FX>),
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
            Scoped::new(scope.clone(), node.target.unwrap()),
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
                Scoped::new(scope.clone(), argexpr.unwrap()),
                Some(Scoped::new(scope, PendingArg(v))),
            ),
            PendingArg(target) => Produce(Scoped::new(scope, target).apply(v)),
        }
    }
}
