use crate::Error;
use chumsky::primitive::just;
use chumsky::Parser;

pub(crate) fn ws() -> impl Parser<char, (), Error = Error> + Clone {
    just(' ').or(just('\n')).repeated().at_least(1).ignored()
}
