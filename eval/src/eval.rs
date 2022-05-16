use crate::scope::ScopeRef;
use crate::{List, Object, Result, ValRef, Value};
use saplang_east::{
    Application, Expr, GenExpr, Identifier, LetExpr, Literal, ObjectDef, PureEffects,
};

pub fn eval(src: &str) -> Result<ValRef> {
    let astexpr = saplang_parser::parse(src)?;
    let expr = Expr::from(astexpr);
    expr.eval(ScopeRef::default())
}

trait Eval {
    fn eval(&self, scope: ScopeRef) -> Result<ValRef>;
}

trait EvalV {
    fn eval_val(&self, scope: ScopeRef) -> Result<Value>;
}

impl<T> Eval for T
where
    T: EvalV,
{
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        self.eval_val(scope).map(ValRef::from)
    }
}

impl<FX> Eval for GenExpr<FX>
where
    FX: Eval,
{
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        use GenExpr::*;

        match &self {
            Lit(x) => x.eval(scope),
            Ref(x) => x.eval(scope),
            List(x) => x.eval(scope),
            Let(x) => x.eval(scope),
            Apply(x) => x.eval(scope),
            Object(x) => x.eval(scope),
            Effect(x) => x.eval(scope),
        }
    }
}

impl Eval for PureEffects {
    fn eval(&self, _scope: ScopeRef) -> Result<ValRef> {
        unreachable!("There are no pure effects beyond `GenExpr` so theis should never evaluate.");
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
            Value::Object(Object { func: Some(fnbox) }) => fnbox(aval),
            _ => Err(Uncallable(tval)),
        }
    }
}

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
