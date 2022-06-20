use super::{Eval, EvalV};
use crate::scope::ScopeRef;
use crate::{Result, ValRef, Value};
use sappho_east::{GenExpr, Literal};

impl<FX> Eval for GenExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        use GenExpr::*;

        match &self {
            Lit(x) => x.eval(scope),
            Ref(x) => scope.deref(x),
            Object(x) => x.eval(scope),
            List(x) => x.eval(scope),
            Let(x) => x.eval(scope),
            Application(x) => x.eval(scope),
            Lookup(x) => x.eval(scope),
            Effect(x) => x.eval(scope),
        }
    }
}

impl EvalV for Literal {
    fn eval_val(&self, _scope: ScopeRef) -> Result<Value> {
        Ok(match self {
            Literal::Num(f) => Value::Num(*f),
        })
    }
}
