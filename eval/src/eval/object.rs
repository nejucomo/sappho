use super::{Eval, EvalV};
use crate::scope::ScopeRef;
use crate::{Object, Result, ValRef, Value};
use sappho_east::ObjectDef;
use sappho_identmap::IdentMap;

impl EvalV for ObjectDef {
    fn eval_val(&self, scope: ScopeRef) -> Result<Value> {
        let mut attrs = IdentMap::default();
        for (id, attrexpr) in self.attrs.iter() {
            let v = attrexpr.eval(scope.clone())?;
            attrs.define(id.clone(), v).unwrap();
        }

        let func = self
            .func
            .as_ref()
            .map(|fc| -> Box<dyn Fn(ValRef) -> Result<ValRef>> {
                let defscope = scope.clone();
                let binding = fc.binding.clone();
                let body = fc.body.clone();

                Box::new(move |arg| {
                    let callscope = defscope.extend(&binding, arg);
                    body.eval(callscope)
                })
            });

        let query = self
            .query
            .as_ref()
            .map(|qexpr| -> Box<dyn Fn() -> Result<ValRef>> {
                let body = qexpr.body.clone();
                Box::new(move || body.eval(scope.clone()))
            });

        Ok(Value::Object(Object::new(func, query, attrs)))
    }
}
