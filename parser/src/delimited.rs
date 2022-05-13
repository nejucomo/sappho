use crate::space::ws;
use crate::Error;
use chumsky::primitive::just;
use chumsky::Parser;

pub(crate) fn delimited<P, O>(
    open: char,
    body: P,
    close: char,
) -> impl Parser<char, O, Error = Error>
where
    P: Parser<char, O, Error = Error>,
{
    let bracket = |c| just(c).then_ignore(ws().or_not());

    body.delimited_by(bracket(open), bracket(close))
}
