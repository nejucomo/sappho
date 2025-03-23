use chumsky::Parser as _;
use derive_more::From;
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::primitive::space;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::leftassoc::LeftAssoc;
use crate::parseparams::ParseParams;
use crate::{Confined, Lookups};

#[derive(Debug, PartialEq, From)]
#[from(LeftAssoc<Lookups<FX>, Application<FX>>, Confined<FX>)]
pub struct Applications<FX>(LeftAssoc<Lookups<FX>, Application<FX>>)
where
    FX: Effect;

#[derive(Debug, PartialEq, From)]
#[from(Lookups<FX>, Confined<FX>)]
pub struct Application<FX>(Lookups<FX>)
where
    FX: Effect;

impl_from_via_confined!(Application<FX>);

impl<FX> Applications<FX>
where
    FX: Effect,
{
    pub fn new<L, A, AI>(lookups: L, applications: AI) -> Self
    where
        Lookups<FX>: From<L>,
        Application<FX>: From<A>,
        AI: IntoIterator<Item = A>,
    {
        Applications(LeftAssoc::new(lookups, applications))
    }
}

impl<FX> ParsableWith<ParseParams<'_>> for Applications<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pp: ParseParams<'_>) -> impl Parser<Self> {
        LeftAssoc::parser_with(pp).map(Self)
    }
}

impl<FX> ParsableWith<ParseParams<'_>> for Application<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        space().ignore_then(Lookups::parser_with(pep)).map(Self)
    }
}

impl<FX> Unparse for Applications<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

impl<FX> Unparse for Application<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(" ");
        self.0.unparse_into(s);
    }
}

impl<FX> RestrictFrom<Applications<ProcEffect>> for Applications<FX>
where
    FX: Effect,
{
    fn restrict(src: Applications<ProcEffect>) -> Result<Applications<FX>, Restriction> {
        LeftAssoc::restrict(src.0).map(Applications)
    }
}

impl<FX> RestrictFrom<Application<ProcEffect>> for Application<FX>
where
    FX: Effect,
{
    fn restrict(src: Application<ProcEffect>) -> Result<Application<FX>, Restriction> {
        Lookups::restrict(src.0).map(Application)
    }
}
