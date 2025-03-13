use chumsky::Parser as _;
use sappho_keyword::Keyword::Query as KwQuery;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parserext::ParserExt;
use crate::{ProcExpr, QueryDef, QueryExpr, SEParser};

impl ParsableWith<SEParser<'_>> for QueryDef {
    fn make_parser_with(sep: SEParser<'_>) -> impl Parser<Self> {
        KwQuery
            .parse()
            .then_space()
            .ignore_then(QueryExpr::parser_with(sep))
            .map(Self)
    }
}

impl Parsable for QueryExpr {
    fn parser() -> impl Parser<Self> {
        ProcExpr::parser().restrict()
    }
}

impl ParsableWith<SEParser<'_>> for QueryExpr {
    fn make_parser_with(sep: SEParser<'_>) -> impl Parser<Self> {
        ProcExpr::parser_with(sep).restrict()
    }
}

impl Unparse for QueryDef {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&KwQuery);
        s.write(" ");
        s.write(&self.0);
    }
}

impl Unparse for QueryExpr {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
