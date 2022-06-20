use crate::{
    ApplicationExpr, AstFxFor, FromFx, Identifier, LetExpr, ListForm, Literal, Lookup, ObjectDef,
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
    Apply(ApplicationExpr<Effects>),
    Lookup(Lookup<Effects>),
    Effect(Effects),
}

impl<FX> From<AGE<AstFxFor<FX>>> for GenExpr<FX>
where
    FX: FromFx,
{
    fn from(x: AGE<AstFxFor<FX>>) -> Self {
        use GenExpr as EGE;

        match x {
            AGE::Lit(x) => EGE::Lit(x),
            AGE::Ref(x) => EGE::Ref(x),
            AGE::Func(x) => EGE::Object(ObjectDef::from(x)),
            AGE::Query(x) => EGE::Object(ObjectDef::from(x)),
            AGE::Object(x) => EGE::Object(ObjectDef::from(x)),
            AGE::List(x) => EGE::List(x.into_iter().map(GenExpr::from).collect()),
            AGE::Let(x) => EGE::Let(LetExpr::from(x)),
            AGE::Apply(x) => EGE::Apply(ApplicationExpr::from(x)),
            AGE::Lookup(x) => EGE::Lookup(Lookup::from(x)),
            AGE::Effect(x) => EGE::Effect(FX::from_fx(x)),
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
            Apply(x) => x.fmt(f),
            Lookup(x) => x.fmt(f),
            Effect(x) => x.fmt(f),
        }
    }
}
