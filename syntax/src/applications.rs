use std::rc::Rc;

use chumsky::Parser as _;
use derive_more::From;
use sappho_attrs::Attrs;
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_kast::ProcWiseParser;
use sappho_listform::ListForm;
use sappho_parsable::primitive::space;
use sappho_parsable::{ParsableWith, Parser};
use sappho_primval::Num;
use sappho_unparse::{Stream, Unparse};

use crate::leftassoc::LeftAssoc;
use crate::{
    BoxWise, FuncDef, Interactions, Lookups, ObjectDef, ProcDef, QueryDef, SyntaxProvider, Wise,
};

#[derive(Clone, Debug, PartialEq, From)]
#[from(
    LeftAssoc<Lookups<FX>, Application<FX>>,
    Lookups<FX>,
    Interactions<FX>,
    &'static str,
    Num,
    ObjectDef<FX>,
    Rc<FuncDef>,
    Rc<QueryDef>,
    Rc<ProcDef>,
    Attrs<Wise<FX>>,
    ListForm<Wise<FX>, BoxWise<FX>>,
)]
pub struct Applications<FX>(LeftAssoc<Lookups<FX>, Application<FX>>)
where
    FX: Effect;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn unwrap(self) -> LeftAssoc<Lookups<FX>, Application<FX>> {
        self.0
    }
}

impl<FX> Application<FX>
where
    FX: Effect,
{
    pub fn unwrap(self) -> Lookups<FX> {
        self.0
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

impl<FX> ParsableWith<ProcWiseParser<'_, SyntaxProvider>> for Applications<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pp: ProcWiseParser<'_, SyntaxProvider>) -> impl Parser<Self> {
        LeftAssoc::parser_with(pp).map(Self)
    }
}

impl<FX> ParsableWith<ProcWiseParser<'_, SyntaxProvider>> for Application<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ProcWiseParser<'_, SyntaxProvider>) -> impl Parser<Self> {
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
