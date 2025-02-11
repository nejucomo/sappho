use std::fmt::Debug;

use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

pub trait AstProvider {
    type Pattern: Unparse + Debug + Clone + PartialEq;

    type Expr<FX>: Unparse + Debug + Clone + PartialEq
    where
        FX: Effect;
}
