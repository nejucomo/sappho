use super::{Eval, EvalV};
use crate::bind;
use crate::scope::ScopeRef;
use crate::{List, Result, ValRef, Value};
use sappho_east::{ApplicationExpr, GenExpr, LetExpr, ListForm, LookupExpr, MatchExpr};

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

impl<FX> Eval for LetExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        let LetExpr {
            binding,
            bindexpr,
            tail,
        } = &self;

        let bindval = bindexpr.eval(scope)?;
        let subscope = scope.extend(binding, bindval);

        tail.eval(&subscope)
    }
}

impl<FX> Eval for MatchExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use crate::Error::Mismatch;

        let MatchExpr { target, clauses } = &self;

        let tval = target.eval(scope)?;
        for clause in clauses {
            if let Some(matchscope) = bind(&clause.pattern, &tval, scope) {
                return clause.body.eval(&matchscope);
            }
        }

        Err(Mismatch(
            tval,
            clauses.iter().map(|c| c.pattern.clone()).collect(),
        ))
    }
}

impl<FX> Eval for ApplicationExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use crate::Func;

        let ApplicationExpr { target, argument } = self;
        let tval = target.eval(scope)?;
        let aval = argument.eval(scope)?;
        let func: &Func = tval.coerce()?;
        func.apply(&aval)
    }
}

impl<FX> Eval for LookupExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: &ScopeRef) -> Result<ValRef> {
        use crate::Attrs;
        use crate::Error::MissingAttr;

        let LookupExpr { target, attr } = self;
        let tval = target.eval(scope)?;
        let attrs: &Attrs = tval.coerce()?;
        attrs
            .get(attr)
            .cloned()
            .ok_or_else(|| MissingAttr(tval, attr.clone()))
    }
}
