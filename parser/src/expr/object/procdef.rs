use crate::error::BareError;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{Ast, ProcExpr};
use sappho_ast_core::{ProcDef, Statements};

pub(crate) fn proc_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcDef<Ast>, Error = BareError> + '_ {
    use crate::delimited::delimited;
    use crate::keyword::Keyword;

    Keyword::Proc
        .parser()
        .ignore_then(delimited('{', statements(expr), '}'))
        .map(ProcDef::from)
}

pub(crate) fn statements(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, Statements<Ast>, Error = BareError> + '_ {
    use crate::keyword::Keyword;
    use crate::space::ws;
    use chumsky::primitive::just;

    Keyword::Return
        .parser()
        .ignore_then(expr)
        .then_ignore(ws().or_not().then(just(';')))
        .map(Box::new)
        .map(Statements::Return)
}
