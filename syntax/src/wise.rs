use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_source::SourceCodeLink;
use sappho_unparse::{Stream, Unparse};
use sappho_with_source::WithSource;

use crate::parseparams::ParseParams;
use crate::parserext::ParserExt as _;
use crate::Wise;

impl<'a, FX> ParsableWith<&'a SourceCodeLink> for Wise<FX>
where
    FX: Effect,
{
    fn make_parser_with(sclink: &'a SourceCodeLink) -> impl Parser<Self> {
        make_proc_wise_parser(sclink).restrict()
    }
}

fn make_proc_wise_parser(sclink: &SourceCodeLink) -> impl Parser<Wise<ProcEffect>> + '_ {
    chumsky::recursive::recursive(|recp| {
        WithSource::parser_with((sclink, ParseParams { recp })).map(Wise)
    })
}

/// This impl terminates recursion within the parser layer
impl<FX> ParsableWith<ParseParams<'_>> for Wise<FX>
where
    FX: Effect,
{
    fn make_parser_with(pp: ParseParams<'_>) -> impl Parser<Self> {
        pp.recp.restrict()
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

impl<FX> RestrictFrom<Wise<ProcEffect>> for Wise<FX>
where
    FX: Effect,
{
    fn restrict(src: Wise<ProcEffect>) -> Result<Wise<FX>, Restriction> {
        WithSource::restrict(src.0).map(Wise)
    }
}
