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
use std::path::PathBuf;

pub fn eval(path: Option<PathBuf>, src: &str) -> Result<ValRef> {
    let astexpr = saplang_parser::parse(path, src)?;
    let expr = PureExpr::from(astexpr);
    expr.eval(ScopeRef::default())
}
