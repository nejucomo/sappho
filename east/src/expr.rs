use crate::{
    ApplicationExpr, AstFxFor, FromFx, Identifier, LetExpr, Literal, LookupExpr, MatchExpr,
    ObjectDef,
};
use sappho_ast as ast;
use sappho_gast as gast;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum GenExpr<Effects> {
    Lit(Literal),
    Ref(Identifier),
    Object(ObjectDef<Effects>),
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
            ast::GenExpr::List(x) => x.into(),
            ast::GenExpr::Let(x) => Let(x.transform_into()),
            ast::GenExpr::Match(x) => Match(x.transform_into()),
            ast::GenExpr::Application(x) => Application(x.transform_into()),
            ast::GenExpr::Lookup(x) => Lookup(x.transform_into()),
            ast::GenExpr::Effect(x) => Effect(FX::from_fx(x)),
        }
    }
}

impl<FX> From<gast::ListForm<ast::GenExpr<AstFxFor<FX>>>> for GenExpr<FX>
where
    FX: FromFx,
{
    fn from(x: gast::ListForm<ast::GenExpr<AstFxFor<FX>>>) -> Self {
        use GenExpr::Object;

        x.into_iter()
            .rev()
            .fold(Object(ObjectDef::default()), |tail, astexpr| {
                Object(ObjectDef::new_attrs([
                    ("head".to_string(), GenExpr::from(astexpr)),
                    ("tail".to_string(), tail),
                ]))
            })
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
            Object(x) => {
                use ast::GenExpr::{Func, Object, Query};
                use sappho_gast::Unbundled as U;
                match x.unbundle() {
                    U::Bundled(obj) => Object(obj.transform_into()),
                    U::Func(f) => Func(f.transform_into()),
                    U::Query(q) => Query(q.transform_into()),
                }
            }
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
            Let(x) => x.fmt(f),
            Match(x) => x.fmt(f),
            Application(x) => x.fmt(f),
            Lookup(x) => x.fmt(f),
            Effect(x) => x.fmt(f),
        }
    }
}
