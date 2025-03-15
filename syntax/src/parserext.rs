use sappho_ast_effect::RestrictFrom;
use sappho_parsable::error::ChumskyError;
use sappho_parsable::Parser;

pub(crate) trait ParserExt<O>: Parser<O> {
    fn restrict<D>(self) -> impl Parser<D>
    where
        D: RestrictFrom<O>,
    {
        self.try_map(|out, span| D::restrict(out).map_err(|err| ChumskyError::custom(span, err)))
    }
}

impl<P, O> ParserExt<O> for P where P: Parser<O> {}
