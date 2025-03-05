use crate::Parser;

pub trait Syntax {
    type Parsed;

    fn into_parser(self) -> impl Parser<Self::Parsed>;
}
