use std::fmt::Debug;

use sappho_effect::Effect;
use sappho_parsable::Parser;
use sappho_source::SourceCodeLink;
use sappho_unparse::Unparse;

use crate::Wise;

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
    type Expr<FX>: ExprDerivableTraits + Unparse
    where
        FX: Effect;

    fn make_wise_parser<FX>(sclink: Option<&SourceCodeLink>) -> impl Parser<Wise<Self, FX>>
    where
        FX: Effect;
}

/// Traits which expressions must extend
pub trait ExprDerivableTraits: Clone + Debug + PartialEq {}

impl<T> ExprDerivableTraits for T where T: Clone + Debug + PartialEq {}
