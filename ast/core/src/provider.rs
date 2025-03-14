use std::fmt::Debug;

use sappho_ast_effect::Effect;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_unparse::Unparse;

pub trait AstProvider<FX>: AstNode
where
    FX: Effect,
{
    type Pattern: AstNode;
}

pub trait AstNode:
    Unparse + Debug + Clone + PartialEq + From<RcId> + From<f64> + From<Attrs<Self>>
{
}

impl<T> AstNode for T where
    T: Unparse + Debug + Clone + PartialEq + From<RcId> + From<f64> + From<Attrs<Self>>
{
}
