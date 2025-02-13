use sappho_ast_core::AstProvider;
use sappho_ast_effect::Effect;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ast;

impl AstProvider for Ast {
    type Pattern = crate::Pattern;

    type Expr<FX>
        = crate::Expr<FX>
    where
        FX: Effect;
}
