use crate::{Eval, EvalThunk, Result};
use sappho_fmtutil::DisplayDepth;
use sappho_value::{GenThunk, ValRef};

impl<FX> EvalThunk for GenThunk<FX>
where
    FX: Eval + DisplayDepth,
{
    fn eval_thunk(&self) -> Result<ValRef> {
        let (expr, defscope) = self.peek();
        expr.eval(defscope)
    }
}
