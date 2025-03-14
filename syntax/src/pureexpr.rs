use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::parserext::ParserExt;
use crate::{ProcExpr, PureExpr};

impl ParsableWith<ParseParams<'_>> for PureExpr {
    fn make_parser_with(sep: ParseParams<'_>) -> impl Parser<Self> {
        ProcExpr::parser_with(sep).restrict()
    }
}

impl Unparse for PureExpr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
