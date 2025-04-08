use chumsky::Parser as _;
use sappho_effect::Effect;
use sappho_kast::KastProvider;
use sappho_parsable::ParsableWith;
use sappho_parsable::Parser;
use sappho_source::SourceCodeLink;
use sappho_syntax as syntax;

use crate::{from_syntax, Expr, Wise};

#[derive(Clone, Debug, PartialEq)]
pub struct EastProvider;

impl KastProvider for EastProvider {
    type Expr<FX>
        = Expr<FX>
    where
        FX: Effect;

    fn make_wise_parser<FX>(sclink: Option<&SourceCodeLink>) -> impl Parser<Wise<FX>>
    where
        FX: Effect,
    {
        syntax::Wise::parser_with(sclink).map(from_syntax)
    }
}
