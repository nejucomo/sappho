use chumsky::Parser as _;
use derive_more::From;
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_listform::ListForm;
use sappho_parsable::primitive::space;
use sappho_parsable::{ParsableWith, Parser};
use sappho_primval::Num;
use sappho_unparse::{Stream, Unparse};

use crate::leftassoc::LeftAssoc;
use crate::parseparams::ParseParams;
use crate::{BoxWise, Interactions, Lookups, Wise};

#[derive(Debug, PartialEq, From)]
#[from(
    LeftAssoc<Lookups<FX>, Application<FX>>,
    Interactions<FX>,
    Lookups<FX>,
    &'static str,
    Num,
    ListForm<Wise<FX>, BoxWise<FX>>,
)]
pub struct Applications<FX>(LeftAssoc<Lookups<FX>, Application<FX>>)
where
    FX: Effect;

#[derive(Debug, PartialEq)]
pub struct Application<FX>(Lookups<FX>)
where
    FX: Effect;

impl<FX> Applications<FX>
where
    FX: Effect,
{
    pub fn new<L, R, RI>(left: L, rights: RI) -> Self
    where
        Lookups<FX>: From<L>,
        Application<FX>: From<R>,
        RI: IntoIterator<Item = R>,
    {
        Applications(LeftAssoc::new(left, rights))
    }
}

impl<FX, T> From<T> for Application<FX>
where
    FX: Effect,
    Lookups<FX>: From<T>,
{
    fn from(t: T) -> Self {
        Application(Lookups::from(t))
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
