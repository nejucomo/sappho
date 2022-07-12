mod unpack;

use crate::{Identifier, Literal};
use sappho_listform::ListForm;
use sappho_unparse::{DisplayDepth, FmtResult, Formatter};

pub use self::unpack::UnpackPattern;

pub type ListPattern = ListForm<Pattern, Identifier>;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(Identifier),
    LitEq(Literal),
    Unpack(UnpackPattern),
    List(ListPattern),
}

impl DisplayDepth for Pattern {
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        use Pattern::*;

        match self {
            Bind(x) => x.fmt_depth(f, depth),
            LitEq(x) => x.fmt_depth(f, depth),
            Unpack(x) => x.fmt_depth(f, depth),
            List(x) => x.fmt_depth(f, depth),
        }
    }
}
