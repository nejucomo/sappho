use sappho_east::{Expr, Let};
use sappho_effect::Effect;

use crate::eval::Evaluatable;
use crate::expr::ContExpr;
use crate::step::Step;

impl<FX> Evaluatable<Step<Expr<FX>, ContExpr<FX>>> for Let<FX>
where
    FX: Effect,
{
    fn eval(self) -> Step<Expr<FX>, ContExpr<FX>> {
        let step: Step<Expr<FX>, ContLet<FX>> = self.eval();
        step.map_continuation(ContExpr::from)
    }
}

impl<FX> Evaluatable<Step<Expr<FX>, ContLet<FX>>> for Let<FX>
where
    FX: Effect,
{
    fn eval(self) -> Step<Expr<FX>, ContLet<FX>> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct ContLet<FX>
where
    FX: Effect,
{
    scope: Scope,
    let_remaining: Let<FX>,
}
