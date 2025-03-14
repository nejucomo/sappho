use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::{ParsableWith, Parser};
use sappho_source::SourceCodeLink;
use sappho_unparse::{Stream, Unparse};
use sappho_with_source::WithSource;

use crate::parseparams::ParseParams;
use crate::{BoxWise, Wise};

/// This impl enables [ParsableWith::load_and_parse]
impl<'a> ParsableWith<&'a SourceCodeLink> for Wise<ProcEffect> {
    fn make_parser_with(sclink: &'a SourceCodeLink) -> impl Parser<Self> {
        chumsky::recursive::recursive(|recp| {
            WithSource::parser_with((
                sclink,
                Self::parser_with(ParseParams {
                    recp,
                    sclink: sclink.clone(),
                }),
            ))
            .map(Self)
        })
    }
}

/// This impl terminates recursion within the parser layer
impl ParsableWith<ParseParams<'_>> for Wise<ProcEffect> {
    fn make_parser_with(pp: ParseParams<'_>) -> impl Parser<Self> {
        pp.recp
    }
}

impl<T> ParsableWith<T> for BoxWise<ProcEffect>
where
    Wise<ProcEffect>: ParsableWith<T>,
{
    fn make_parser_with(param: T) -> impl Parser<Self> {
        Wise::parser_with(param).map(Box::new).map(Self)
    }
}

impl<FX> Unparse for BoxWise<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

impl<FX> Unparse for Wise<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
