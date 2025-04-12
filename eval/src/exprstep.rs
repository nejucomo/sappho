use std::error::Error;

use easy_ext::ext;
use sappho_continuation::ContinueStep;
use sappho_continuation::Step::{self, Produce};
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_primval::PrimVal;
use sappho_value::Value;

use crate::eval::EvalResult;
use crate::expr::ContExpr;

pub type ExprStep<'s, FX> = Step<EvalResult<Value>, &'s Expr<FX>, ContExpr<'s, FX>>;

#[ext(ExprContinue)]
impl<'s, FX> &'s Expr<FX>
where
    FX: Effect,
{
    fn continue_to<C>(self, cont: C) -> ExprStep<'s, FX>
    where
        C: Into<ContExpr<'s, FX>>,
    {
        ContinueStep {
            expr: WithLocals::new(self, None),
            cont: cont.into(),
        }
        .into()
    }
}

impl<'s, FX> From<Value> for ExprStep<'s, FX>
where
    FX: Effect,
{
    fn from(value: Value) -> Self {
        Produce(Ok(value))
    }
}

impl<'s, FX> From<PrimVal> for ExprStep<'s, FX>
where
    FX: Effect,
{
    fn from(value: PrimVal) -> Self {
        Produce(Ok(value.into()))
    }
}

impl<'s, FX> From<&PrimVal> for ExprStep<'s, FX>
where
    FX: Effect,
{
    fn from(value: &PrimVal) -> Self {
        Self::from(*value)
    }
}

impl<'s, FX, T, E> From<Result<T, E>> for ExprStep<'s, FX>
where
    FX: Effect,
    T: Into<Value>,
    E: Error + 'static,
{
    fn from(value: Result<T, E>) -> Self {
        Produce(value.map(T::into).map_err(Box::from))
    }
}
