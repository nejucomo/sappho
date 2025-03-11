use chumsky::Parser as _;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_primval::PrimVal;
use sappho_unparse::Unparse;

use crate::Pattern::{self, *};

impl Parsable for Pattern {
    fn parser() -> impl Parser<Self> {
        chumsky::recursive::recursive(Pattern::parser_with)
    }
}

impl ParsableWith<Recursive<'_, Pattern>> for Pattern {
    fn make_parser_with(pattern: Recursive<'_, Pattern>) -> impl Parser<Self> {
        RcId::parser()
            .map(Bind)
            .or(PrimVal::parser().map(LitEq))
            .or(Attrs::parse_with(pattern.clone()).map(Unpack))
            .or(ListForm::parse_with((pattern, RcId::parser())).map(Unpack))
    }
}

impl Unparse for Pattern {
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            Bind(x) => x.unparse_into(s),
            LitEq(x) => x.unparse_into(s),
            Unpack(x) => x.unparse_into(s),
            List(x) => x.unparse_into(s),
        }
    }
}
