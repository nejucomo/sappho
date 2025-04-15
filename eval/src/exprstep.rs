use sappho_east::Expr;
use sappho_value::Value;

use crate::contexpr::ContExpr;
use crate::step::Step;

pub(crate) type ExprStep<'x, FX> = Step<Value, &'x Expr<FX>, ContExpr<'x, FX>>;
