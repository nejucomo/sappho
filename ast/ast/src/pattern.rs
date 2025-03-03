use crate::Literal;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_unparse::{Stream, Unparse};

pub type ListPattern = ListForm<Pattern, RcId>;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(RcId),
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
