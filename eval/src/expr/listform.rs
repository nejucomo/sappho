use crate::{Eval, EvalV, Result};
use sappho_east::{GenExpr, ListForm};
use sappho_value::{List, ScopeRef, Value};

impl<FX> EvalV for ListForm<GenExpr<FX>>
where
    FX: Eval,
{
    fn eval_val(&self, scope: &ScopeRef) -> Result<Value> {
        eval_list_slice(self.as_ref(), scope).map(Value::from)
    }
}

fn eval_list_slice<FX>(exprs: &[GenExpr<FX>], scope: &ScopeRef) -> Result<List>
where
    FX: Eval,
{
    if exprs.is_empty() {
        Ok(List::default())
    } else {
        let head = exprs[0].eval(scope)?;
        let tail = eval_list_slice(&exprs[1..], scope)?;
        Ok(tail.prepend(head))
    }
}
