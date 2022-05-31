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
                let binding = fc.binding.clone();
                let body = fc.body.clone();

                Box::new(move |arg| {
                    let callscope = scope.extend(&binding, arg);
                    body.eval(callscope)
                })
            });

        Ok(Value::Object(Object { func, attrs }))
    }
}
