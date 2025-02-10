use sappho_ast_core::ExprProvider;
use sappho_ast_effect::Effect;

pub struct AstProvider;

impl ExprProvider for AstProvider {
    type Pattern = crate::Pattern;

    type Expr<FX>
        = crate::Expr<FX>
    where
        FX: Effect;
}
