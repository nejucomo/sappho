use derive_more::From;
use sappho_effect::Effect;
use sappho_value::Value;

use crate::evaltrait::Eval;
use crate::exprstep::ExprStep;
use crate::listdef::ContListDef;
use crate::objectdef::ContObjectDef;

/// The primary continuation for [crate::eval]
#[derive(Debug, From)]
pub(crate) enum ContExpr<'x, FX>
where
    FX: Effect,
{
    ObjectDef(ContObjectDef<'x, FX>),
    ListDef(ContListDef<'x, FX>),
}

impl<'x, FX> Eval<Value, ExprStep<'x, FX>> for ContExpr<'x, FX>
where
    FX: Effect,
{
    fn eval(self, input: Value) -> ExprStep<'x, FX> {
        use ContExpr::*;

        match self {
            ObjectDef(x) => x.eval(input),
            ListDef(x) => x.eval(input),
        }
    }
}
