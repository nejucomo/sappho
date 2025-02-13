use std::fmt::Debug;

use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

use crate::{AstRed, AstTransformInto, CoreExpr, CorePattern};

/// # Note
///
/// The supertraits [Debug], [Clone], and [PartialEq] are a hack to enable derivation for container types that wrap [AstProvider::Expr] or [AstProvider::Pattern].
///
/// TODO: Replace hand-written impls of the above with derivations.
pub trait AstProvider: Debug + Clone + PartialEq {
    type Pattern: AstTransformInto<CorePattern> + Unparse + Debug + Clone + PartialEq;

    type Expr<FX>: AstTransformInto<CoreExpr<AstRed, FX>> + Unparse + Debug + Clone + PartialEq
    where
        FX: Effect;
}
