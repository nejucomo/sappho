use std::error::Error;

use sappho_continuation::Step::{self, Produce};
use sappho_east::Expr;
use sappho_effect::Effect;
use sappho_primval::PrimVal;
use sappho_value::Value;

use crate::eval::EvalResult;
use crate::expr::ContExpr;
use crate::wrapper::Ev;

pub type ExprStep<'s, FX> = Step<EvalResult<Value>, Ev<&'s Expr<FX>>, ContExpr<'s, FX>>;
