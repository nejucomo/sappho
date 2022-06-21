mod effects;
mod expr;
mod object;
mod recursive;
mod traits;

pub(crate) use self::traits::Eval;
use self::traits::EvalV;
use crate::Result;
use sappho_east::PureExpr;
use sappho_value::{ScopeRef, ValRef};

pub fn eval<AST>(ast: AST) -> Result<ValRef>
where
    PureExpr: From<AST>,
{
    let expr = PureExpr::from(ast);
    expr.eval(&ScopeRef::default())
}
