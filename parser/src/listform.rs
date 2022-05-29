use crate::delimited::delimited;
use crate::error::BareError;
use crate::space::ws;
use chumsky::primitive::just;
use chumsky::Parser;

pub(crate) fn list_form<P, O>(item: P) -> impl Parser<char, Vec<O>, Error = BareError>
where
    P: Parser<char, O, Error = BareError>,
{
    delimited(
        '[',
        item.separated_by(just(',').then_ignore(ws().or_not())),
        ']',
    )
}
