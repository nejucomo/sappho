use crate::{
    ApplicationExpr, AstFxFor, FromFx, Identifier, LetExpr, ListForm, Literal, LookupExpr,
    MatchExpr, ObjectDef,
};
use sappho_ast::GenExpr as AGE;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum GenExpr<Effects> {
    Lit(Literal),
    Ref(Identifier),
    Object(ObjectDef),
    List(ListForm<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Match(MatchExpr<Effects>),
    Application(ApplicationExpr<Effects>),
    Lookup(LookupExpr<Effects>),
    Effect(Effects),
}

impl<FX> From<AGE<AstFxFor<FX>>> for GenExpr<FX>
where
    FX: FromFx,
{
    fn from(x: AGE<AstFxFor<FX>>) -> Self {
        use GenExpr::*;

        match x {
            AGE::Lit(x) => Lit(x),
            AGE::Ref(x) => Ref(x),
            AGE::Func(x) => Object(ObjectDef::from(x)),
            AGE::Query(x) => Object(ObjectDef::from(x)),
            AGE::Object(x) => Object(ObjectDef::from(x)),
            AGE::List(x) => List(x.into_iter().map(GenExpr::from).collect()),
            AGE::Let(x) => Let(LetExpr::from(x)),
            AGE::Match(x) => Match(MatchExpr::from(x)),
            AGE::Application(x) => Application(ApplicationExpr::from(x)),
            AGE::Lookup(x) => Lookup(LookupExpr::from(x)),
            AGE::Effect(x) => Effect(FX::from_fx(x)),
        }
    }
}

impl<FX> fmt::Display for GenExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GenExpr::*;

        match self {
            Lit(x) => x.fmt(f),
            Ref(x) => x.fmt(f),
            Object(x) => x.fmt(f),
            List(x) => x.fmt(f),
            Let(x) => x.fmt(f),
            Match(x) => x.fmt(f),
            Application(x) => x.fmt(f),
            Lookup(x) => x.fmt(f),
            Effect(x) => x.fmt(f),
        }
    }
}
