use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(RcId),
    LitEq(PrimVal),
    Unpack(UnpackPattern),
    List(ListPattern),
}

pub type UnpackPattern = Attrs<Pattern>;
pub type ListPattern = ListForm<Pattern, RcId>;

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
