use crate::{trace_eval, Result, ValRef};
use sappho_east::PureExpr;
use sappho_value::ScopeRef;

/// Evaluate an `AST` into a [ValRef] or an [Error](crate::Error).
///
/// `AST` can be anything that converts to [sappho_east::PureExpr], including
/// `sappho_ast::PureExpr` which is the output of `sappho_parser::parse`.
pub fn eval<AST>(ast: AST) -> Result<ValRef>
where
    PureExpr: From<AST>,
{
    let expr = PureExpr::from(ast);
    trace_eval(&expr, &ScopeRef::default())
}
