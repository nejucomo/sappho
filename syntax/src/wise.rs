use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::{ParsableWith, Parser};
use sappho_source::SourceCodeLink;
use sappho_unparse::{Stream, Unparse};
use sappho_with_source::WithSource;

use crate::parseparams::ParseParams;
use crate::parserext::ParserExt as _;
use crate::restrict::RestrictInto;
use crate::{BoxWise, Wise};

impl<'a, FX> ParsableWith<&'a SourceCodeLink> for Wise<FX>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
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
    ProcEffect: RestrictInto<FX>,
{
    fn make_parser_with(pp: ParseParams<'_>) -> impl Parser<Self> {
        pp.recp.restrict()
    }
}

impl<T, FX> ParsableWith<T> for BoxWise<FX>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
    Wise<FX>: ParsableWith<T>,
{
    fn make_parser_with(param: T) -> impl Parser<Self> {
        Wise::<FX>::parser_with(param).map(Box::new).map(Self)
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
