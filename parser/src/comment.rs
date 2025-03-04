use chumsky::Parser;
use sappho_ast_comment::CmtPrefixed;

use crate::error::BareError;

pub(crate) fn cmt_prefixed<P, T>(p: P) -> impl Parser<char, CmtPrefixed<T>, Error = BareError>
where
    P: Parser<char, T, Error = BareError>,
{
    // TODO: Actually parse comments
    p.map(|t| CmtPrefixed::new(None, t))
}
