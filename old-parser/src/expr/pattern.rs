use crate::error::BareError;
use crate::expr::universal::{identifier, literal};
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{ListPattern, Pattern};
use sappho_attrs::Attrs;

pub(crate) fn pattern() -> impl Parser<char, Pattern, Error = BareError> {
    chumsky::recursive::recursive(pattern_rec)
}

fn pattern_rec(
    pat: Recursive<'_, char, Pattern, BareError>,
) -> impl Parser<char, Pattern, Error = BareError> + '_ {
    use Pattern::*;

    identifier()
        .map(Bind)
        .or(literal().map(LitEq))
        .or(unpack_attrs(pat.clone()).map(Unpack))
        .or(list_pattern(pat).map(List))
        .labelled("pattern")
}

fn unpack_attrs(
    pat: Recursive<'_, char, Pattern, BareError>,
) -> impl Parser<char, Attrs<Pattern>, Error = BareError> + '_ {
    use crate::delimited::delimited;
    use crate::space::ws;
    use chumsky::primitive::just;

    delimited(
        '{',
        identifier()
            .then_ignore(just(':').then(ws().or_not()))
            .then(pat)
            .separated_by(just(',').then(ws().or_not()))
            .allow_trailing(),
        '}',
    )
    .map(Attrs::from_iter)
}

fn list_pattern(
    pat: Recursive<'_, char, Pattern, BareError>,
) -> impl Parser<char, ListPattern, Error = BareError> + '_ {
    use crate::listform::list_form;

    list_form(pat, identifier()).labelled("list-pattern")
}
