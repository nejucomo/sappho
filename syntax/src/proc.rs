use chumsky::Parser as _;
use sappho_keyword::Keyword::Proc as KwProc;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{ProcDef, ProcExpr, SEParser, BSE};

impl Parsable for ProcExpr {
    fn parser() -> impl Parser<Self> {
        BSE::parser().map(Self)
    }
}

impl ParsableWith<SEParser<'_>> for ProcExpr {
    fn make_parser_with(sep: SEParser<'_>) -> impl Parser<Self> {
        BSE::parser_with(sep).map(Self)
    }
}

impl ParsableWith<SEParser<'_>> for ProcDef {
    fn make_parser_with(sep: SEParser<'_>) -> impl Parser<Self> {
        KwProc
            .parse()
            .then_space()
            .ignore_then(bracketed(['{', '}'], ProcExpr::parser_with(sep)))
            .map(Self)
    }
}

impl Unparse for ProcExpr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s);
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
