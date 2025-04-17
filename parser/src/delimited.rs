use chumsky::primitive::just;
use chumsky::Parser as _;
use sappho_parsable::primitive::space;
use sappho_parsable::Parser;

pub(crate) fn delimited<P, O>(open: char, body: P, close: char) -> impl Parser<O>
where
    P: Parser<O>,
{
    body.delimited_by(
        just(open).then_ignore(space().or_not()),
        space().or_not().ignore_then(just(close)),
    )
}
