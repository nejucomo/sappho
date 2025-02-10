use sappho_ast_core::ExprProvider;
use sappho_ast_effect::Effect;

use crate::{Expr, Pattern};

pub struct AstProvider;

impl ExprProvider for AstProvider {
    type Pattern = Pattern;

    type Expr<FX>
        = Expr<FX>
    where
        FX: Effect;
}
