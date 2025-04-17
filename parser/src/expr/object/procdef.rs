use crate::error::BareError;
use chumsky::recursive::Recursive;
use chumsky::Parser as _;
use sappho_ast::{Ast, ProcExpr};
use sappho_ast_core::{ProcDef, Statements};
use sappho_keyword::Keyword;
use sappho_parsable::primitive::space;
use sappho_parsable::Parser;

pub(crate) fn proc_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<ProcDef<Ast>> + '_ {
    use crate::delimited::delimited;

    Keyword::Proc
        .parse()
        .ignore_then(delimited('{', statements(expr), '}'))
        .map(ProcDef::from)
}

pub(crate) fn statements(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<Statements<Ast>> + '_ {
    use chumsky::primitive::just;
    use sappho_keyword::Keyword;

    Keyword::Return
        .parse()
        .ignore_then(expr)
        .then_ignore(space().or_not().then(just(';')))
        .map(Box::new)
        .map(Statements::Return)
}
