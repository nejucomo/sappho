use chumsky::Parser as _;
use sappho_keyword::Keyword::Query as KwQuery;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::proc::ProcExprParser;
use crate::restrict::RestrictInto;
use crate::{ProcExpr, QueryDef, QueryExpr};

impl ParsableWith<ProcExprParser<'_>> for QueryDef {
    fn make_parser_with(proc_expr: ProcExprParser<'_>) -> impl Parser<Self> {
        KwQuery
            .parse()
            .then_space()
            .ignore_then(Box::parser_with(proc_expr))
            .map(Self)
    }
}

impl ParsableWith<ProcExprParser<'_>> for QueryExpr {
    fn make_parser_with(pep: ProcExprParser<'_>) -> impl Parser<Self> {
        ProcExpr::parser_with(pep).try_map(|px, span| px.restrict(span))
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
