use crate::{
    ApplicationExpr, AstFxFor, FromFx, Identifier, LetExpr, ListForm, Literal, LookupExpr,
    MatchExpr, ObjectDef,
};
use sappho_ast as ast;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
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

impl<FX> From<ast::GenExpr<AstFxFor<FX>>> for GenExpr<FX>
where
    FX: FromFx,
{
    fn from(x: ast::GenExpr<AstFxFor<FX>>) -> Self {
        use GenExpr::*;

        match x {
            ast::GenExpr::Lit(x) => Lit(x),
            ast::GenExpr::Ref(x) => Ref(x),
            ast::GenExpr::Func(x) => ast::GenExpr::from(ast::ObjectDef::new_func(x)).into(),
            ast::GenExpr::Query(x) => ast::GenExpr::from(ast::ObjectDef::new_query(x)).into(),
            ast::GenExpr::Object(x) => Object(x.transform_into()),
            ast::GenExpr::List(x) => List(x.into_iter().map(GenExpr::from).collect()),
            ast::GenExpr::Let(x) => Let(x.transform_into()),
            ast::GenExpr::Match(x) => Match(x.transform_into()),
            ast::GenExpr::Application(x) => Application(x.transform_into()),
            ast::GenExpr::Lookup(x) => Lookup(x.transform_into()),
            ast::GenExpr::Effect(x) => Effect(FX::from_fx(x)),
        }
    }
}

impl<FX> From<GenExpr<AstFxFor<FX>>> for ast::GenExpr<FX>
where
    FX: FromFx,
{
    fn from(x: GenExpr<AstFxFor<FX>>) -> Self {
        use GenExpr::*;

        match x {
            Lit(x) => ast::GenExpr::Lit(x),
            Ref(x) => ast::GenExpr::Ref(x),
            Object(x) => ast::GenExpr::Object(x.transform_into()),
            List(x) => ast::GenExpr::List(x.into_iter().map(ast::GenExpr::from).collect()),
            Let(x) => ast::GenExpr::Let(x.transform_into()),
            Match(x) => ast::GenExpr::Match(x.transform_into()),
            Application(x) => ast::GenExpr::Application(x.transform_into()),
            Lookup(x) => ast::GenExpr::Lookup(x.transform_into()),
            Effect(x) => ast::GenExpr::Effect(FX::from_fx(x)),
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
