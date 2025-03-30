use crate::{Eval, EvalThunk, Result};
use sappho_effect::Effect;
use sappho_ast_kernel::Interaction;
use sappho_ast_red::AstRed;
use sappho_value::{GenThunk, ValRef};

impl<FX> EvalThunk for GenThunk<FX>
where
    Interaction<AstRed, FX>: Eval,
    FX: Effect,
{
    fn eval_thunk(&self) -> Result<ValRef> {
        let (expr, defscope) = self.peek();
        expr.eval(defscope)
    }
}
