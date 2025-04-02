use either::Either::Right;
use sappho_east::{BoxWise, Let, LetClause};
use sappho_effect::Effect;
use sappho_pattern::Pattern;
use sappho_scope::Locals;
use sappho_value::Value;

use crate::continuation::Continuation;
use crate::step::{EvalNext, EvalStep, State, Step};

impl<FX> EvalStep<FX> for Let<FX>
where
    FX: Effect,
{
    type Continuation = ContLet<FX>;

    fn eval_step(self, state: State) -> Step<FX, Self::Continuation> {
        StepLet {
            locals: Locals::default(),
            clauses: self.clauses.into_iter(),
            inner: self.inner,
        }
        .eval_step(state)
    }
}

#[derive(Debug)]
pub(crate) struct StepLet<FX>
where
    FX: Effect,
{
    locals: Locals,
    clauses: <Vec<LetClause<FX>> as IntoIterator>::IntoIter,
    inner: BoxWise<FX>,
}

impl<FX> EvalStep<FX> for StepLet<FX>
where
    FX: Effect,
{
    type Continuation = ContLet<FX>;

    fn eval_step(self, state: State) -> Step<FX, Self::Continuation> {
        if let Some(clause) = self.clauses.next() {
            xxx
        } else {
            Right(EvalNext::new_ws(state, self.inner, None))
        }
    }
}

#[derive(Debug)]
pub(crate) struct ContLet<FX>
where
    FX: Effect,
{
    binding: Pattern,
    steplet: StepLet<FX>,
}

impl<FX> Continuation<FX> for ContLet<FX>
where
    FX: Effect,
{
    type EvalStep = StepLet<FX>;

    fn continue_eval(self, v: Value) -> Self::EvalStep {
        let ContLet {
            mut binding,
            steplet,
        } = self;

        steplet.locals.bind(binding, v).unwrap();
        steplet
    }
}
