use crate::error::BareError;
use crate::expr::universal::literal;
use chumsky::recursive::Recursive;
use chumsky::{text, Parser};
use sappho_ast::{ListPattern, Pattern, UnpackPattern};

pub(crate) fn pattern() -> impl Parser<char, Pattern, Error = BareError> {
    chumsky::recursive::recursive(pattern_rec)
}

fn pattern_rec(
    pat: Recursive<'_, char, Pattern, BareError>,
) -> impl Parser<char, Pattern, Error = BareError> + '_ {
    use Pattern::*;

    text::ident()
        .map(Bind)
        .or(literal().map(LitEq))
        .or(unpack_attrs(pat.clone()).map(Unpack))
        .or(list_pattern(pat).map(List))
        .labelled("pattern")
}

fn unpack_attrs(
    pat: Recursive<'_, char, Pattern, BareError>,
) -> impl Parser<char, UnpackPattern, Error = BareError> + '_ {
    use crate::delimited::delimited;
    use crate::space::ws;
    use chumsky::primitive::just;

    delimited(
        '{',
        text::ident()
            .then_ignore(just(':').then(ws().or_not()))
            .then(pat)
            .separated_by(just(',').then(ws().or_not()))
            .allow_trailing(),
        '}',
    )
    .map(UnpackPattern::from_iter)
}

fn list_pattern(
    pat: Recursive<'_, char, Pattern, BareError>,
) -> impl Parser<char, ListPattern, Error = BareError> + '_ {
    use crate::delimited::delimited;
    use crate::space::ws;
    use chumsky::primitive::just;

    delimited(
        '[',
        pat.separated_by(just(',').then(ws().or_not()))
            .allow_trailing(),
        ']',
    )
    .map(ListPattern::from_iter)
}
