use crate::scope::ScopeRef;
use crate::{FuncObj, List, Result, ValRef, Value};
use saplang_ast::{Application, Expr, FuncExpr, Identifier, LetExpr, Literal};

pub fn eval(src: &str) -> Result<ValRef> {
    let expr = saplang_parser::parse(src)?;
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

impl Eval for Expr {
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        use Expr::*;

        match &self {
            Lit(x) => x.eval(scope),
            Ref(x) => x.eval(scope),
            List(x) => x.eval(scope),
            Let(x) => x.eval(scope),
            Func(x) => x.eval(scope),
            Apply(x) => x.eval(scope),
            Object(x) => todo!("{:?}", x),
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

impl EvalV for Vec<Expr> {
    fn eval_val(&self, scope: ScopeRef) -> Result<Value> {
        eval_list_slice(&self[..], scope).map(Value::from)
    }
}

fn eval_list_slice(exprs: &[Expr], scope: ScopeRef) -> Result<List> {
    if exprs.is_empty() {
        Ok(List::default())
    } else {
        let head = exprs[0].eval(scope.clone())?;
        let tail = eval_list_slice(&exprs[1..], scope)?;
        Ok(tail.prepend(head))
    }
}

impl Eval for LetExpr {
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

impl EvalV for FuncExpr {
    fn eval_val(&self, scope: ScopeRef) -> Result<Value> {
        let binding = self.binding.clone();
        let body = self.body.clone();

        Ok(Value::Func(FuncObj(Box::new(move |arg| {
            let callscope = scope.extend(&binding, arg);
            body.eval(callscope)
        }))))
    }
}

impl Eval for Application {
    fn eval(&self, scope: ScopeRef) -> Result<ValRef> {
        use crate::Error::Uncallable;
        use std::borrow::Borrow;

        let Application { target, argument } = self;
        let tval = target.eval(scope.clone())?;
        let aval = argument.eval(scope)?;
        match tval.borrow() {
            Value::Func(FuncObj(fnbox)) => fnbox(aval),
            _ => Err(Uncallable(tval)),
        }
    }
}
