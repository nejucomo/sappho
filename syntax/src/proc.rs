use chumsky::Parser as _;
use sappho_keyword::Keyword::Proc as KwProc;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_unparse::{Stream, Unparse};

use crate::spanned::Spanned;
use crate::{ProcDef, ProcExpr};

impl Parsable for ProcExpr {
    fn parser() -> impl Parser<Self> {
        chumsky::recursive::recursive(ProcExpr::parser_with)
    }
}

pub(crate) type ProcExprParser<'a> = Recursive<'a, ProcExpr>;

impl ParsableWith<ProcExprParser<'_>> for ProcExpr {
    fn make_parser_with(pep: ProcExprParser<'_>) -> impl Parser<Self> {
        Spanned::parser_with(pep).map(ProcExpr)
    }
}

impl ParsableWith<ProcExprParser<'_>> for ProcDef {
    fn make_parser_with(pep: ProcExprParser<'_>) -> impl Parser<Self> {
        KwProc
            .parse()
            .then_space()
            .ignore_then(bracketed(['{', '}'], Box::parser_with(pep)))
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
