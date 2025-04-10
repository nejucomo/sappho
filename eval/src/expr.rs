use derive_more::From;
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_value::{Scope, Value};

use crate::continuation::{Continuation, EvalStep};
use crate::letexpr::LetCont;
use crate::listdef::ListDefCont;
use crate::objectdef::ObjDefCont;
use crate::step::Step;

#[derive(Debug, From)]
pub(crate) enum ExprCont<'a, FX>
where
    FX: Effect,
{
    Let(LetCont<FX>),
    ObjDef(ObjDefCont<'a, FX>),
    ListDef(ListDefCont<FX>),
}

impl<FX> EvalStep<FX> for Expr<FX>
where
    FX: Effect,
{
    type Continuation<'a> = ExprCont<'a, FX>;

    fn eval_step<'a>(&'a self, scope: &Scope) -> Step<'a, FX, Self::Continuation<'a>> {
        use Expr::*;
        use Step::*;

        match self {
            Prim(x) => Produce(Value::from(*x)),
            Ref(x) => Produce(
                scope
                    .lookup(&x)
                    .expect("TODO: user-space exceptions")
                    .clone(),
            ),
            ObjectDef(x) => x.eval_step(scope).cont_from(),
            ListDef(x) => x.eval_step(scope).cont_from(),
            Let(x) => x.eval_step(scope).cont_from(),
            Application(x) => x.eval_step(scope).cont_from(),
            other => todo!("{other:?}"),
        }
    }
}

impl<FX> Continuation<FX> for ExprCont<FX>
where
    FX: Effect,
{
    fn eval_from_value<'a>(&'a self, v: Value) -> Step<'a, FX, Self> {
        use ExprCont::*;

        match self {
            Let(x) => x.eval_from_value(v).cont_from(),
            ObjDef(x) => x.eval_from_value(v).cont_from(),
            ListDef(x) => x.eval_from_value(v).cont_from(),
        }
    }
}
