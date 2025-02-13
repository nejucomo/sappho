use sappho_ast_effect::Effect;

use crate::{AstProvider, CoreExpr, CorePattern};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AstCore;

impl AstProvider for AstCore {
    type Pattern = CorePattern;

    type Expr<FX>
        = CoreExpr<AstCore, FX>
    where
        FX: Effect;
}
