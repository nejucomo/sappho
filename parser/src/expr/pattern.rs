use crate::error::BareError;
use crate::expr::universal::identifier;
use chumsky::recursive::Recursive;
use chumsky::Parser as _;
use sappho_ast::{ListPattern, Pattern};
use sappho_attrs::Attrs;
use sappho_parsable::primitive::space;
use sappho_parsable::{Parsable as _, Parser};
use sappho_primval::PrimVal;

pub(crate) fn pattern() -> impl Parser<Pattern> {
    chumsky::recursive::recursive(pattern_rec)
}

fn pattern_rec(pat: Recursive<'_, char, Pattern, BareError>) -> impl Parser<Pattern> + '_ {
    use Pattern::*;

    identifier()
        .map(Bind)
        .or(PrimVal::parser().map(LitEq))
        .or(unpack_attrs(pat.clone()).map(Unpack))
        .or(list_pattern(pat).map(List))
        .labelled("pattern")
}

fn unpack_attrs(pat: Recursive<'_, char, Pattern, BareError>) -> impl Parser<Attrs<Pattern>> + '_ {
    use crate::delimited::delimited;
    use chumsky::primitive::just;

    delimited(
        '{',
        identifier()
            .then_ignore(just(':').then(space().or_not()))
            .then(pat)
            .separated_by(just(',').then(space().or_not()))
            .allow_trailing(),
        '}',
    )
    .map(Attrs::from_iter)
}

fn list_pattern(pat: Recursive<'_, char, Pattern, BareError>) -> impl Parser<ListPattern> + '_ {
    use crate::listform::list_form;

    list_form(pat, identifier()).labelled("list-pattern")
}
