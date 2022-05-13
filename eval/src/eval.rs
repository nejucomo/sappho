use crate::{Result, Value};
use saplang_ast::Expr;

pub fn eval(src: &str) -> Result<Value> {
    let expr = saplang_parser::parse(src)?;
    eval_expr(expr)
}

fn eval_expr(expr: Expr) -> Result<Value> {
    use saplang_ast::Literal::Num;

    match expr {
        Expr::Lit(Num(f)) => Ok(Value::Num(f)),
        _ => todo!("{:?}", expr),
    }
}
