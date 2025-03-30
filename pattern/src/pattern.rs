use chumsky::Parser as _;
use derive_more::{From, TryInto};
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_parsable::{Parsable, ParsableWith, Parser, Recursive};
use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

use crate::BindPattern;

use self::Pattern::*;

#[derive(Clone, Debug, PartialEq, From, TryInto)]
pub enum Pattern {
    #[from(BindPattern, RcId, &'static str)]
    Bind(BindPattern),
    #[from]
    LitEq(PrimVal),
    #[from]
    Unpack(Attrs<Pattern>),
    #[from]
    List(ListForm<Pattern, BindPattern>),
}

impl Parsable for Pattern {
    fn parser() -> impl Parser<Self> {
        chumsky::recursive::recursive(|pattern| {
            BindPattern::parser_with(pattern.clone())
                .map(Bind)
                .or(PrimVal::parser().map(LitEq))
                .or(Attrs::parser_with(pattern.clone()).map(Unpack))
                .or(ListForm::parser_with(pattern).map(List))
        })
    }
}

impl ParsableWith<Recursive<'_, Pattern>> for Pattern {
    fn make_parser_with(pattern: Recursive<'_, Pattern>) -> impl Parser<Self> {
        pattern
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
