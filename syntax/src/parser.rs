use crate::Error;

pub trait Parser<Output>: chumsky::Parser<char, Output, Error = Error> {}

impl<P, O> Parser<O> for P where P: chumsky::Parser<char, O, Error = Error> {}
