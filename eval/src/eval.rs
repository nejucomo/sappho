use sappho_east::Expr;
use sappho_effect::ProcEffect;
use sappho_value::Value;

pub fn eval<X>(expr: X) -> Value
where
    X: Into<Expr<ProcEffect>>,
{
    expr.into().eval()
}

pub trait Evaluatable<Output> {
    fn eval(self) -> Output;
}
