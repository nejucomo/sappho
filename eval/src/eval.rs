use crate::{Eval, Result};
use sappho_east::PureExpr;
use sappho_value::{ScopeRef, ValRef};

pub fn eval<AST>(ast: AST) -> Result<ValRef>
where
    PureExpr: From<AST>,
{
    let expr = PureExpr::from(ast);
    expr.eval(&ScopeRef::default())
}
