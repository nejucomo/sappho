mod unpack;

use crate::{Identifier, Literal};
use sappho_listform::ListForm;
use sappho_unparse::{Unparse, Stream};

pub use self::unpack::UnpackPattern;

pub type ListPattern = ListForm<Pattern, Identifier>;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(Identifier),
    LitEq(Literal),
    Unpack(UnpackPattern),
    List(ListPattern),
}

impl Unparse for Pattern {
    fn unparse_into(&self, s: &mut Stream) {
        use Pattern::*;

        match self {
            Bind(x) => x.unparse(f, depth),
            LitEq(x) => x.unparse(f, depth),
            Unpack(x) => x.unparse(f, depth),
            List(x) => x.unparse(f, depth),
        }
    }
}
