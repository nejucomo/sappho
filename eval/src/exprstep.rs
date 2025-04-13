use sappho_continuation::Step;
use sappho_east::Expr;
use sappho_value::Value;

use crate::eval::EvalResult;
use crate::expr::ContExpr;
use crate::wrapper::Ev;

pub type ExprStep<'s, FX> = Step<EvalResult<Value>, Ev<&'s Expr<FX>>, ContExpr<'s, FX>>;
