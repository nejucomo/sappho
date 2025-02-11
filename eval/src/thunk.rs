use crate::{Eval, EvalThunk, Result};
use sappho_ast::Effect;
use sappho_ast_reduced::EffectExpr;
use sappho_value::{GenThunk, ValRef};

impl<FX> EvalThunk for GenThunk<FX>
where
    EffectExpr<FX>: Eval,
    FX: Effect,
{
    fn eval_thunk(&self) -> Result<ValRef> {
        let (expr, defscope) = self.peek();
        expr.eval(defscope)
    }
}
