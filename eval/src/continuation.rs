use sappho_east::{BoxWise, Expr, Wise};
use sappho_effect::Effect;
use sappho_value::Value;
use sappho_with_source::WithSource;

use crate::expr::ExprCont;
use crate::scoped::Scoped;
use crate::step::Step;

pub(crate) trait EvalStep<FX>: Sized
where
    FX: Effect,
{
    type Continuation: Continuation<FX>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation>;
}

pub(crate) trait Continuation<FX>: Sized
where
    FX: Effect,
{
    fn eval_from_value(self, v: Value) -> Step<Scoped<Wise<FX>>, Self>;
}

impl<FX> EvalStep<FX> for Scoped<BoxWise<FX>>
where
    FX: Effect,
{
    type Continuation = ExprCont<FX>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation> {
        self.map(BoxWise::unwrap).eval_step()
    }
}

impl<FX> EvalStep<FX> for Wise<FX>
where
    FX: Effect,
{
    type Continuation = ExprCont<FX>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation> {
        Scoped::from(self).eval_step()
    }
}

impl<FX> EvalStep<FX> for Scoped<Wise<FX>>
where
    FX: Effect,
{
    type Continuation = ExprCont<FX>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation> {
        self.map(Wise::unwrap).eval_step()
    }
}

/// # TODO
///
/// Use the source code to annotate error messages. Currently there is no error handling.
impl<FX> EvalStep<FX> for Scoped<WithSource<Expr<FX>>>
where
    FX: Effect,
{
    type Continuation = ExprCont<FX>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation> {
        self.map(WithSource::ignore_source).eval_step()
    }
}
