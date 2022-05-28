use crate::error::BareError;
use chumsky::primitive::just;
use chumsky::Parser;

pub(crate) fn ws() -> impl Parser<char, (), Error = BareError> + Clone {
    just(' ').or(just('\n')).repeated().at_least(1).ignored()
}
