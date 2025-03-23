use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_more::From;
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_identifier::RcId;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::leftassoc::LeftAssoc;
use crate::parseparams::ParseParams;
use crate::Interactions;

#[derive(Debug, PartialEq, From)]
pub struct Lookups<FX>(LeftAssoc<Interactions<FX>, Lookup>)
where
    FX: Effect;

#[derive(Debug, PartialEq, From)]
pub struct Lookup(RcId);

impl<FX> ParsableWith<ParseParams<'_>> for Lookups<FX>
where
    FX: Effect,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        LeftAssoc::parser_with(pep).map(Self)
    }
}

impl ParsableWith<ParseParams<'_>> for Lookup {
    fn make_parser_with(_: ParseParams<'_>) -> impl Parser<Self> {
        just('.').ignore_then(RcId::parser()).map(Self)
    }
}

impl<FX> Unparse for Lookups<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s);
    }
}

impl Unparse for Lookup {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(".");
        self.0.unparse_into(s);
    }
}

impl<FX> RestrictFrom<Lookups<ProcEffect>> for Lookups<FX>
where
    FX: Effect,
{
    fn restrict(src: Lookups<ProcEffect>) -> Result<Lookups<FX>, Restriction> {
        LeftAssoc::restrict(src.0).map(Lookups)
    }
}

impl RestrictFrom<Lookup> for Lookup {
    fn restrict(src: Lookup) -> Result<Lookup, Restriction> {
        Ok(src)
    }
}
