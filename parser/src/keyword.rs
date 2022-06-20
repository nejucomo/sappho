use crate::error::BareError;
use crate::space::ws;
use chumsky::primitive::just;
use chumsky::Parser;

#[derive(Copy, Clone, Debug)]
pub enum Keyword {
    Fn,
    Let,
    Match,
    Proc,
    Query,
}

impl Keyword {
    pub fn iter() -> impl Iterator<Item = Self> {
        Iter(Some(Keyword::Fn))
    }

    pub fn as_str(self) -> &'static str {
        use Keyword::*;

        match self {
            Fn => "fn",
            Let => "let",
            Match => "match",
            Proc => "proc",
            Query => "query",
        }
    }

    pub fn parser(self) -> impl Parser<char, (), Error = BareError> {
        just(self.as_str()).then_ignore(ws()).ignored()
    }
}

struct Iter(Option<Keyword>);

impl Iterator for Iter {
    type Item = Keyword;

    fn next(&mut self) -> Option<Keyword> {
        let r = self.0.take();
        self.0 = r.and_then(|kw| {
            use Keyword::*;

            match kw {
                Fn => Some(Let),
                Let => Some(Match),
                Match => Some(Proc),
                Proc => Some(Query),
                Query => None,
            }
        });
        r
    }
}
