use enum_iterator::Sequence;
use sappho_parsable::Parser;
use sappho_unparse::Unparse;

#[derive(Copy, Clone, Debug, Sequence, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Let,
    Match,
    Proc,
    Query,
    Return,
}

impl Keyword {
    pub fn each() -> impl Iterator<Item = Self> {
        enum_iterator::all()
    }

    pub fn as_str(self) -> &'static str {
        use Keyword::*;

        match self {
            Fn => "fn",
            Let => "let",
            Match => "match",
            Proc => "proc",
            Query => "query",
            Return => "return",
        }
    }

    pub fn parse(self) -> impl Parser<Self> {
        use chumsky::prelude::just;
        use chumsky::Parser as _;

        just(self.as_str()).then_space().to(self)
    }
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        for kw in Keyword::each() {
            if kw.as_str() == s {
                return Ok(kw);
            }
        }
        Err(())
    }
}

impl AsRef<str> for Keyword {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Unparse for Keyword {
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        s.write(self.as_str())
    }
}
