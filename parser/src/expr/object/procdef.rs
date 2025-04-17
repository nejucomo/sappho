use crate::error::BareError;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{Ast, ProcExpr};
use sappho_ast_core::{ProcDef, Statements};
use sappho_keyword::Keyword;
use sappho_parsable::primitive::space;

pub(crate) fn proc_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcDef<Ast>, Error = BareError> + '_ {
    use crate::delimited::delimited;

    Keyword::Proc
        .parse()
        .ignore_then(delimited('{', statements(expr), '}'))
        .map(ProcDef::from)
}

pub(crate) fn statements(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, Statements<Ast>, Error = BareError> + '_ {
    use chumsky::primitive::just;
    use sappho_keyword::Keyword;

    Keyword::Return
        .parse()
        .ignore_then(expr)
        .then_ignore(space().or_not().then(just(';')))
        .map(Box::new)
        .map(Statements::Return)
}
