use chumsky::Parser as _;
use derive_more::From;
use sappho_identifier::RcId;
use sappho_parsable::{Parsable as _, ParsableWith, Parser, Recursive};
use sappho_unparse::{Stream, Unparse};

use crate::Pattern;

#[derive(Clone, Debug, PartialEq, From)]
#[from(RcId, &'static str)]
pub struct BindPattern(RcId);

impl ParsableWith<Recursive<'_, Pattern>> for BindPattern {
    fn make_parser_with(_: Recursive<'_, Pattern>) -> impl Parser<Self> {
        RcId::parser().map(BindPattern::from)
    }
}

impl Unparse for BindPattern {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
