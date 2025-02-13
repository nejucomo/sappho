use sappho_ast_effect::Effect;

use crate::{AstProvider, CorePattern, RedExpr};

/// The [AstCore] provider is the minimal AST that is evaluated
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AstRed;

impl AstProvider for AstRed {
    type Pattern = CorePattern;

    type Expr<FX>
        = RedExpr<FX>
    where
        FX: Effect;
}
