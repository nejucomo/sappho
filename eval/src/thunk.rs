use crate::{Eval, EvalThunk, Result};
use sappho_value::{GenThunk, ValRef};

impl<'a, FX> EvalThunk for GenThunk<'a, FX>
where
    FX: Eval,
{
    fn eval_thunk(&self) -> Result<ValRef> {
        let (expr, defscope) = self.peek();
        expr.eval(defscope)
    }
}
