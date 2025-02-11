use std::fmt::Debug;

use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

/// # Note
///
/// The supertraits [Debug], [Clone], and [PartialEq] are a hack to enable derivation for container types that wrap [ExprProvider::Expr] or [ExprProvider::Pattern].
///
/// TODO: Replace hand-written impls of the above with derivations.
pub trait AstProvider: Debug + Clone + PartialEq {
    type Pattern: Unparse + Debug + Clone + PartialEq;

    type Expr<FX>: Unparse + Debug + Clone + PartialEq
    where
        FX: Effect;
}
