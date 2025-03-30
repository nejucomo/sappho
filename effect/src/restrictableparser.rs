use sappho_parsable::error::ChumskyError;
use sappho_parsable::Parser;

use crate::RestrictFrom;

pub trait RestrictableParser<O>: Parser<O> {
    fn restricted<D>(self) -> impl Parser<D>
    where
        D: RestrictFrom<O>,
    {
        self.try_map(|out, span| D::restrict(out).map_err(|err| ChumskyError::custom(span, err)))
    }
}

impl<P, O> RestrictableParser<O> for P where P: Parser<O> {}
