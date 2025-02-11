use sappho_ast_core::ExprProvider;
use sappho_ast_effect::Effect;

#[derive(Copy, Clone, Default, Debug)]
pub struct AstReduced;

impl ExprProvider for AstReduced {
    type Pattern = crate::Pattern;

    type Expr<FX>
        = crate::Expr<FX>
    where
        FX: Effect;
}
