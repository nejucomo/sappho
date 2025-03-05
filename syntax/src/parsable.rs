use crate::Parser;

pub trait Parsable: Sized {
    fn parser() -> impl Parser<Self>;
}
