use crate::{Eval, EvalThunk, Result};
use sappho_ast::Effect;
use sappho_ast_core::EffectExpr;
use sappho_ast_reduced::AstRed;
use sappho_value::{GenThunk, ValRef};

impl<FX> EvalThunk for GenThunk<FX>
where
    EffectExpr<AstRed, FX>: Eval,
    FX: Effect,
{
    fn eval_thunk(&self) -> Result<ValRef> {
        let (expr, defscope) = self.peek();
        expr.eval(defscope)
    }
}
