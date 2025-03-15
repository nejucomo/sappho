use sappho_ast_effect::Effect;
use sappho_ast_kernel::AstProvider;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Ast;

impl AstProvider for Ast {
    type Pattern = crate::Pattern;

    type Expr<FX>
        = crate::Expr<FX>
    where
        FX: Effect;
}
