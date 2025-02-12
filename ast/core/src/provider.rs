use std::fmt::Debug;

use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

use crate::{CoreExpr, CorePattern};

/// # Note
///
/// The supertraits [Debug], [Clone], and [PartialEq] are a hack to enable derivation for container types that wrap [AstProvider::Expr] or [AstProvider::Pattern].
///
/// TODO: Replace hand-written impls of the above with derivations.
pub trait AstProvider: Debug + Clone + PartialEq {
    type Pattern: Into<CorePattern> + Unparse + Debug + Clone + PartialEq;

    type Expr<FX>: Into<CoreExpr<AstCore, FX>> + Unparse + Debug + Clone + PartialEq
    where
        FX: Effect;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AstCore;

impl AstProvider for AstCore {
    type Pattern = CorePattern;

    type Expr<FX>
        = CoreExpr<AstCore, FX>
    where
        FX: Effect;
}
