use crate::{Eval, EvalThunk, Result};
use sappho_east::EffectExpr;
use sappho_unparse::Unparse;
use sappho_value::{GenThunk, ValRef};

impl<FX> EvalThunk for GenThunk<FX>
where
    EffectExpr<FX>: Eval,
    FX: Unparse,
{
    fn eval_thunk(&self) -> Result<ValRef> {
        let (expr, defscope) = self.peek();
        expr.eval(defscope)
    }
}
