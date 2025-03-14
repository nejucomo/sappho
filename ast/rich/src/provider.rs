use sappho_ast_effect::Effect;
use sappho_ast_kernel::AstProvider;

#[derive(Debug)]
pub struct Ast;

impl AstProvider for Ast {
    type Pattern = crate::Pattern;

    type Expr<FX>
        = crate::Expr<FX>
    where
        FX: Effect;
}
