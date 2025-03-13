use sappho_parsable::Parser;

use crate::restrict::RestrictInto;

pub(crate) trait ParserExt<O>: Parser<O> {
    fn restrict<D>(self) -> impl Parser<D>
    where
        O: RestrictInto<D>,
    {
        self.try_map(|out, span| out.restrict(span))
    }
}

impl<P, O> ParserExt<O> for P where P: Parser<O> {}
