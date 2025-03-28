use std::fmt::Debug;

use sappho_ast_effect::Effect;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_primval::Num;
use sappho_unparse::Unparse;

/// # Caution
///
/// [AstProvider] extends [AstNodeBase] purely as a hack to enable deriving those traits types which are paremeterized on this trait
pub trait AstProvider: AstNodeBase {
    type Pattern: AstNode;

    type Expr<FX>: AstNode
    where
        FX: Effect;
}

pub trait AstNodeBase: Debug + Clone + PartialEq {}

pub trait AstNode: AstNodeBase + Unparse + From<RcId> + From<Num> + From<Attrs<Self>> {}

// Blanket impls:
impl<T> AstNodeBase for T where T: Debug + Clone + PartialEq {}

impl<T> AstNode for T where T: AstNodeBase + Unparse + From<RcId> + From<Num> + From<Attrs<Self>> {}
