use derive_more::From;
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::Value;

use crate::eval::Evaluatable;
use crate::letexpr::ContLet;
use crate::step::Step;

impl<FX> Evaluatable<Value> for Expr<FX>
where
    FX: Effect,
{
    fn eval(self) -> Value {
        todo!()
    }
}

impl<FX> Evaluatable<Step<Self, ContExpr<FX>>> for Expr<FX>
where
    FX: Effect,
{
    fn eval(self) -> Step<Self, ContExpr<FX>> {
        use Expr::*;

        match self {
            Prim(x) => x.into(),
            Let(x) => x.eval(),
        }
    }
}

#[derive(Debug, From)]
pub(crate) enum ContExpr<FX> {
    Let(ContLet<FX>),
}
