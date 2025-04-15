use sappho_east::Wise;
use sappho_value::Value;

use crate::contexpr::ContExpr;
use crate::step::Step;

pub(crate) type ExprStep<'x, FX> = Step<Value, &'x Wise<FX>, ContExpr<'x, FX>>;
