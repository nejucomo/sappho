use derive_more::From;
use sappho_east::{Expr, Wise};
use sappho_effect::Effect;
use sappho_scope::Scoped;
use sappho_value::Value;

use crate::evco::{Continuation, Eval};
use crate::letexpr::LetCont;
use crate::step::Step;

#[derive(Debug, From)]
pub(crate) enum ExprCont<FX>
where
    FX: Effect,
{
    Let(LetCont<FX>),
}

impl<FX> Eval<FX> for Scoped<Expr<FX>>
where
    FX: Effect,
{
    type Continuation = ExprCont<FX>;

    fn eval_step(self) -> Step<Scoped<Wise<FX>>, Self::Continuation> {
        use Expr::*;
        use Step::*;

        match self.node {
            Prim(x) => Produce(x.into()),
            Let(x) => self.scope.wrap(x).eval_step().cont_from(),
            _ => todo!(),
        }
    }
}

impl<FX> Continuation<FX> for ExprCont<FX>
where
    FX: Effect,
{
    fn eval_from_value(self, v: Value) -> Step<Scoped<Wise<FX>>, Self> {
        use ExprCont::*;

        match self {
            Let(x) => x.eval_from_value(v).cont_from(),
        }
    }
}
