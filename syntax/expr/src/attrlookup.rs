use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_syntax_idstore::ArcId;
use sappho_syntax_parsable::{Parsable, Parser};
use sappho_syntax_unparse::{Stream, Unparse};

#[derive(Clone, Debug, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct AttrLookup(ArcId);

impl Parsable for AttrLookup {
    fn parser() -> impl Parser<Self> {
        just('.').ignore_then(ArcId::parser()).map(AttrLookup)
    }
}

impl Unparse for AttrLookup {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(".");
        s.write(&self.0);
    }
}
