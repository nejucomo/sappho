use sappho_ast_core::ExprProvider;
use sappho_ast_effect::Effect;

#[derive(Debug)]
pub struct AstProvider;

impl ExprProvider for AstProvider {
    type Pattern = crate::Pattern;

    type Expr<FX>
        = crate::Expr<FX>
    where
        FX: Effect;
}
