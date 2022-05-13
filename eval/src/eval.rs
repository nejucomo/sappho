use crate::scope::Scope;
use crate::{Result, ValRef};
use saplang_ast::Expr;

pub fn eval(src: &str) -> Result<ValRef> {
    let expr = saplang_parser::parse(src)?;
    eval_expr(Scope::Empty, expr)
}

fn eval_expr(scope: Scope, expr: Expr) -> Result<ValRef> {
    use crate::Value;
    use saplang_ast::Literal::Num;

    match expr {
        Expr::Lit(Num(f)) => Ok(ValRef::from(Value::Num(f))),
        Expr::Ref(ident) => scope.deref(&ident),
        _ => todo!("{:?}", expr),
    }
}
