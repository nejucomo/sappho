mod effects;
mod expr;
mod object;
mod recursive;
mod traits;
mod universal;

use self::traits::{Eval, EvalV};
use crate::scope::ScopeRef;
use crate::{Result, ValRef};
use saplang_east::PureExpr;

pub fn eval(src: &str) -> Result<ValRef> {
    let astexpr = saplang_parser::parse(src)?;
    let expr = PureExpr::from(astexpr);
    expr.eval(ScopeRef::default())
}
