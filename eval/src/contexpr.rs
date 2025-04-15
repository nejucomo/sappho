use derive_more::From;

use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_primval::PrimVal;
use sappho_value::{Scope, Value};

use crate::evaltrait::Eval;
use crate::exprstep::ExprStep;
use crate::objectdef::ContObjectDef;
use crate::step::Step;

impl<'s, 'x, FX> Eval<&'s Scope, ExprStep<'x, FX>> for &'x Expr<FX>
where
    FX: Effect,
{
    fn eval(self, scope: &'s Scope) -> ExprStep<'x, FX> {
        use Expr::*;

        match self {
            Prim(x) => x.eval(scope),
            Ref(x) => x.eval(scope),
            ObjectDef(x) => x.eval(scope),
            ListDef(x) => x.eval(scope),
            Let(x) => x.eval(scope),
            Match(x) => x.eval(scope),
            Application(x) => x.eval(scope),
            Lookup(x) => x.eval(scope),
            Interaction(x) => x.eval(scope),
        }
    }
}

impl<'x, FX, S> Eval<S, ExprStep<'x, FX>> for &'x PrimVal
where
    FX: Effect,
{
    fn eval(self, _: S) -> ExprStep<'x, FX> {
        Step::produce(*self)
    }
}

impl<'s, 'x, FX> Eval<&'s Scope, ExprStep<'x, FX>> for &'x RcId
where
    FX: Effect,
{
    fn eval(self, scope: &'s Scope) -> ExprStep<'x, FX> {
        Step::produce(
            scope
                .lookup(self)
                .expect("TODO: eval user-space error propagation")
                .clone(),
        )
    }
}

/// The primary continuation for [crate::eval]
#[derive(Debug, From)]
pub(crate) enum ContExpr<'x, FX>
where
    FX: Effect,
{
    Cont(ContObjectDef<'x, FX>),
}

impl<'x, FX> Eval<Value, ExprStep<'x, FX>> for ContExpr<'x, FX>
where
    FX: Effect,
{
    fn eval(self, input: Value) -> ExprStep<'x, FX> {
        use ContExpr::*;

        match self {
            Cont(x) => x.eval(input),
        }
    }
}
