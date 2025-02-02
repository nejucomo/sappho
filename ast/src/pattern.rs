mod unpack;

use crate::{Identifier, Literal};
use sappho_legible::{IntoNode, Node};
use sappho_listform::ListForm;

pub use self::unpack::UnpackPattern;

pub type ListPattern = ListForm<Pattern, Identifier>;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(Identifier),
    LitEq(Literal),
    Unpack(UnpackPattern),
    List(ListPattern),
}

impl IntoNode for &Pattern {
    fn into_node(self) -> Node {
        use Pattern::*;

        match self {
            Bind(x) => x.into_node(),
            LitEq(x) => x.into_node(),
            Unpack(x) => x.into_node(),
            List(x) => x.into_node(),
        }
    }
}
