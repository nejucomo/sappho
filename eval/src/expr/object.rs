use crate::{Eval, EvalV, Result};
use sappho_ast_reduced::{EffectExpr, ObjectDef};
use sappho_value::{Attrs, Func, Object, Proc, Query, ScopeRef, Value};

impl<FX> EvalV for ObjectDef<FX>
where
    EffectExpr<FX>: Eval,
{
    fn eval_val(&self, scope: &ScopeRef) -> Result<Value> {
        let mut attrs = Attrs::default();
        for (id, attrexpr) in self.attrs().iter() {
            let v = attrexpr.eval(scope)?;
            attrs.define(id.clone(), v).unwrap();
        }

        let func = self.func().map(|fc| Func::new(fc, scope));
        let query = self.query().map(|qc| Query::new(qc, scope));
        let proc = self
            .proc()
            .map(|pdef| Proc::new(pdef.clone(), scope.clone()));

        Ok(Value::Object(Box::new(Object::new(
            func, query, proc, attrs,
        ))))
    }
}
