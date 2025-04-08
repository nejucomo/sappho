use chumsky::Parser as _;
use sappho_effect::{Effect, ProcEffect, RestrictableParser as _};
use sappho_kast::{KastProvider, ProcWiseParser};
use sappho_parsable::{ParsableWith as _, Parser};
use sappho_source::SourceCodeLink;
use sappho_with_source::WithSource;

use crate::{Expr, Wise};

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxProvider;

impl KastProvider for SyntaxProvider {
    type Expr<FX>
        = Expr<FX>
    where
        FX: Effect;

    fn make_wise_parser<FX>(sclink: Option<&SourceCodeLink>) -> impl Parser<Wise<FX>>
    where
        FX: Effect,
    {
        chumsky::recursive::recursive(|pwp| inner(sclink, pwp)).restricted()
    }
}

fn inner<'a>(
    sclink: Option<&'a SourceCodeLink>,
    proc: ProcWiseParser<'a, SyntaxProvider>,
) -> impl Parser<Wise<ProcEffect>> + 'a {
    WithSource::parser_with((sclink, proc)).map(Wise::from)
}
