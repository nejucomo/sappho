mod effects;
mod expr;
mod object;
mod recursive;
mod traits;

use self::traits::{Eval, EvalV};
use crate::scope::ScopeRef;
use crate::{Result, ValRef};
use sappho_east::PureExpr;

pub fn eval<AST>(ast: AST) -> Result<ValRef>
where
    PureExpr: From<AST>,
{
    let expr = PureExpr::from(ast);
    expr.eval(ScopeRef::default())
}
