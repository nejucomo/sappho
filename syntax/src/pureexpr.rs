use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parserext::ParserExt;
use crate::{ProcExpr, PureExpr, SEParser};

impl ParsableWith<SEParser<'_>> for PureExpr {
    fn make_parser_with(sep: SEParser<'_>) -> impl Parser<Self> {
        ProcExpr::parser_with(sep).restrict()
    }
}

impl Unparse for PureExpr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
