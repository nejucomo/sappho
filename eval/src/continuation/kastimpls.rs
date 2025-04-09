//! impls of `EvalStep` for `kast` types
use sappho_east::{BoxWise, Expr, Wise};
use sappho_effect::Effect;
use sappho_with_source::WithSource;

use crate::continuation::EvalStep;
use crate::expr::ExprCont;
use crate::scoped::Scoped;
use crate::step::Step;

impl<FX> EvalStep<FX> for Scoped<BoxWise<FX>>
where
    FX: Effect,
{
    type Continuation = ExprCont<FX>;

    fn eval_step<'a>(&'a self, scope: &Scope) -> Step<'a, FX, Self::Continuation> {
        self.map(BoxWise::unwrap).eval_step()
    }
}

impl<FX> EvalStep<FX> for Wise<FX>
where
    FX: Effect,
{
    type Continuation = ExprCont<FX>;

    fn eval_step<'a>(&'a self, scope: &Scope) -> Step<'a, FX, Self::Continuation> {
        Scoped::from(self).eval_step()
    }
}

impl<FX> EvalStep<FX> for Scoped<Wise<FX>>
where
    FX: Effect,
{
    type Continuation = ExprCont<FX>;

    fn eval_step<'a>(&'a self, scope: &Scope) -> Step<'a, FX, Self::Continuation> {
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

    fn eval_step<'a>(&'a self, scope: &Scope) -> Step<'a, FX, Self::Continuation> {
        self.map(WithSource::ignore_source).eval_step()
    }
}
