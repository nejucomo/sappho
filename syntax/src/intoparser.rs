use crate::Parser;

pub trait IntoParser<Output> {
    fn into_parser(self) -> impl Parser<Output>;
}
