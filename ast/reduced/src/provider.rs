use sappho_ast_core::AstProvider;
use sappho_ast_effect::Effect;

/// The [AstProvider] for a "REDuced" AST named [AstRed]
#[derive(Copy, Clone, Default, Debug)]
pub struct AstRed;

impl AstProvider for AstRed {
    type Pattern = crate::Pattern;

    type Expr<FX>
        = crate::Expr<FX>
    where
        FX: Effect;
}
