use super::{Eval, EvalV};
use crate::scope::ScopeRef;
use crate::{Func, Object, Query, Result, Value};
use sappho_east::ObjectDef;
use sappho_identmap::IdentMap;

impl EvalV for ObjectDef {
    fn eval_val(&self, scope: &ScopeRef) -> Result<Value> {
        let mut attrs = IdentMap::default();
        for (id, attrexpr) in self.attrs().iter() {
            let v = attrexpr.eval(scope)?;
            attrs.define(id.clone(), v).unwrap();
        }

        let func = self.func().map(|fc| Func::new(fc, scope));
        let query = self.query().map(|qc| Query::new(qc, scope));

        Ok(Value::Object(Object::new(func, query, attrs)))
    }
}
