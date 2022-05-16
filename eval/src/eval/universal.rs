use super::{Eval, EvalV};
use crate::scope::ScopeRef;
use crate::{Result, ValRef, Value};
use saplang_east::{Identifier, Literal, UniversalExpr};

impl Eval for UniversalExpr {
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        use UniversalExpr::*;

        match &self {
            Lit(x) => x.eval(scope),
            Ref(x) => x.eval(scope),
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

impl Eval for Identifier {
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        scope.deref(self)
    }
}
