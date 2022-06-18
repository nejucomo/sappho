use super::{Eval, EvalV};
use crate::scope::ScopeRef;
use crate::{List, Result, ValRef, Value};
use sappho_east::{Application, GenExpr, LetExpr, RecursiveExpr};

impl<FX> Eval for RecursiveExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        use RecursiveExpr::*;

        match self {
            List(x) => x.eval(scope),
            Let(x) => x.eval(scope),
            Apply(x) => x.eval(scope),
        }
    }
}

impl<FX> EvalV for Vec<GenExpr<FX>>
where
    FX: Eval,
{
    fn eval_val(&self, scope: ScopeRef) -> Result<Value> {
        eval_list_slice(&self[..], scope).map(Value::from)
    }
}

fn eval_list_slice<FX>(exprs: &[GenExpr<FX>], scope: ScopeRef) -> Result<List>
where
    FX: Eval,
{
    if exprs.is_empty() {
        Ok(List::default())
    } else {
        let head = exprs[0].eval(scope.clone())?;
        let tail = eval_list_slice(&exprs[1..], scope)?;
        Ok(tail.prepend(head))
    }
}

impl<FX> Eval for LetExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        let LetExpr {
            binding,
            bindexpr,
            tail,
        } = &self;

        let bindval = bindexpr.eval(scope.clone())?;
        let subscope = scope.extend(binding, bindval);

        tail.eval(subscope)
    }
}

impl<FX> Eval for Application<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        use crate::Error::Uncallable;
        use std::borrow::Borrow;

        let Application { target, argument } = self;
        let tval = target.eval(scope.clone())?;
        let aval = argument.eval(scope)?;
        match tval.borrow() {
            Value::Object(obj) => {
                if let Some(fnbox) = obj.func() {
                    fnbox(aval)
                } else {
                    Err(Uncallable(tval))
                }
            }
            _ => Err(Uncallable(tval)),
        }
    }
}
