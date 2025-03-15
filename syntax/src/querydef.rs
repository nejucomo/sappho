use chumsky::Parser as _;
use sappho_keyword::Keyword::Query as KwQuery;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::{QueryDef, QueryExpr};

impl ParsableWith<ParseParams<'_>> for QueryDef {
    fn make_parser_with(sep: ParseParams<'_>) -> impl Parser<Self> {
        KwQuery
            .parse()
            .then_space()
            .ignore_then(QueryExpr::parser_with(sep))
            .map(Self)
    }
}

impl Unparse for QueryDef {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&KwQuery);
        s.write(" ");
        s.write(&self.0);
    }
}
