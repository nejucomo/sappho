use super::{Eval, EvalV};
use crate::scope::ScopeRef;
use crate::{Object, Result, ValRef, Value};
use sappho_east::ObjectDef;

impl EvalV for ObjectDef {
    fn eval_val(&self, scope: ScopeRef) -> Result<Value> {
        Ok(Value::Object(Object {
            func: self
                .func
                .as_ref()
                .map(|fc| -> Box<dyn Fn(ValRef) -> Result<ValRef>> {
                    let binding = fc.binding.clone();
                    let body = fc.body.clone();

                    Box::new(move |arg| {
                        let callscope = scope.extend(&binding, arg);
                        body.eval(callscope)
                    })
                }),
        }))
    }
}
