use crate::error::BareError;
use crate::keyword::Keyword;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{ProcDef, ProcExpr, Statements};

pub(crate) fn proc_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcDef, Error = BareError> + '_ {
    use crate::delimited::delimited;

    Keyword::Proc
        .parser()
        .ignore_then(delimited('{', statements(expr), '}'))
        .map(ProcDef::from)
}

pub(crate) fn statements(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, Statements, Error = BareError> + '_ {
    Keyword::Return
        .parser()
        .ignore_then(expr)
        .map(Box::new)
        .map(Statements::Return)
}
