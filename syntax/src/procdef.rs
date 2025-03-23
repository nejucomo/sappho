use chumsky::Parser as _;
use derive_more::From;
use sappho_keyword::Keyword::Proc as KwProc;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::ProcExpr;

#[derive(Debug, PartialEq, From)]
pub struct ProcDef(ProcExpr);

impl ParsableWith<ParseParams<'_>> for ProcDef {
    fn make_parser_with(sep: ParseParams<'_>) -> impl Parser<Self> {
        KwProc
            .parse()
            .then_space()
            .ignore_then(bracketed(['{', '}'], ProcExpr::parser_with(sep)))
            .map(Self)
    }
}

impl Unparse for ProcDef {
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::Brackets::Squiggle;
        use sappho_unparse::Break;

        s.write(&KwProc);
        s.write(" ");
        s.bracketed(Squiggle, |subs| {
            subs.write(&Break::Mandatory);
            subs.write(&self.0);
        });
    }
}
