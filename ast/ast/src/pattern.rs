use crate::Literal;
use sappho_attrs::Attrs;
use sappho_listform::ListForm;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_unparse::{Stream, Unparse};

pub type ListPattern = ListForm<Pattern, ArcId>;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(ArcId),
    LitEq(Literal),
    Unpack(Attrs<Pattern>),
    List(ListPattern),
}

impl Unparse for Pattern {
    fn unparse_into(&self, s: &mut Stream) {
        use Pattern::*;

        match self {
            Bind(x) => x.unparse_into(s),
            LitEq(x) => x.unparse_into(s),
            Unpack(x) => x.unparse_into(s),
            List(x) => x.unparse_into(s),
        }
    }
}
