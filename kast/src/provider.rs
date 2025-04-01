use std::fmt::Debug;

use sappho_effect::Effect;

/// The top-level AST provider which embeds KAST components
///
/// # Hack
///
/// Note: this uninhabited trait extends traits which we want to apply to expressions. This is a hack to work-around the `derive` limitation which gates directly on a parameter (rather than the types actually used).
///
/// # TODO:
///
/// Move parsing constraints out of these requirements
pub trait KastProvider: ExprDerivableTraits + 'static {
    type Expr<FX>: ExprDerivableTraits
    where
        FX: Effect;
}

/// Traits which expressions must extend
pub trait ExprDerivableTraits: Clone + Debug + PartialEq {}

impl<T> ExprDerivableTraits for T where T: Clone + Debug + PartialEq {}
