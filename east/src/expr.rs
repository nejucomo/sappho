use crate::{
    ApplicationExpr, EffectExpr, Identifier, LetExpr, Literal, LookupExpr, MatchExpr, ObjectDef,
};
use sappho_ast as ast;
use sappho_gast::transform_object_def;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<Effects> {
    Lit(Literal),
    Ref(Identifier),
    Object(ObjectDef<Effects>),
    Let(LetExpr<Effects>),
    Match(MatchExpr<Effects>),
    Application(ApplicationExpr<Effects>),
    Lookup(LookupExpr<Effects>),
    Effect(EffectExpr<Effects>),
}

impl<FX> From<ast::Expr<FX>> for Expr<FX> {
    fn from(x: ast::Expr<FX>) -> Self {
        use Expr::*;

        match x {
            ast::Expr::Lit(x) => Lit(x),
            ast::Expr::Ref(x) => Ref(x),
            ast::Expr::Func(x) => ast::Expr::from(ast::ObjectDef::new_func(x)).into(),
            ast::Expr::Query(x) => ast::Expr::from(ast::ObjectDef::new_query(x)).into(),
            ast::Expr::Object(x) => Object(transform_object_def(x)),
            ast::Expr::List(x) => x.into(),
            ast::Expr::Let(x) => Let(x.transform_into()),
            ast::Expr::Match(x) => Match(x.transform_into()),
            ast::Expr::Application(x) => Application(x.transform_into()),
            ast::Expr::Lookup(x) => Lookup(x.transform_into()),
            ast::Expr::Effect(x) => Effect(x.transform_into()),
        }
    }
}

impl<FX> From<ast::ListExpr<FX>> for Expr<FX> {
    fn from(x: ast::ListExpr<FX>) -> Self {
        use Expr::Object;

        x.into_reverse_fold(
            |opttail| {
                opttail
                    .map(|x| Expr::from(*x))
                    .unwrap_or_else(|| Object(ObjectDef::default()))
            },
            |tail, head| {
                Object(ObjectDef::new_attrs([
                    ("head".to_string(), Expr::from(head)),
                    ("tail".to_string(), tail),
                ]))
            },
        )
    }
}

impl<FX> From<Expr<FX>> for ast::Expr<FX>
where
    FX: Clone,
{
    fn from(x: Expr<FX>) -> Self {
        use Expr::*;

        match x {
            Lit(x) => ast::Expr::Lit(x),
            Ref(x) => ast::Expr::Ref(x),
            Object(x) => objdef_to_ast_expr(x),
            Let(x) => ast::Expr::Let(x.transform_into()),
            Match(x) => ast::Expr::Match(x.transform_into()),
            Application(x) => ast::Expr::Application(x.transform_into()),
            Lookup(x) => ast::Expr::Lookup(x.transform_into()),
            Effect(x) => ast::Expr::Effect(x.transform_into()),
        }
    }
}

fn objdef_to_ast_expr<FX>(objdef: ObjectDef<FX>) -> ast::Expr<FX>
where
    FX: Clone,
{
    use ast::Expr::{Func, List, Object, Query};
    use sappho_object::Unbundled as U;

    match objdef.unbundle() {
        U::Bundled(obj) => Object(transform_object_def(obj)),
        U::Func(f) => Func(f.transform_into()),
        U::Query(q) => Query(q.transform_into()),
        U::Attrs(a) => a
            .as_list_form()
            .map(|listform| {
                List(
                    listform
                        .map_elems(|x| ast::Expr::from(x.clone()))
                        .map_tail(|x| Box::new(ast::Expr::from(x.clone()))),
                )
            })
            .unwrap_or_else(|| Object(transform_object_def(ObjectDef::new_attrs(a)))),
    }
}

impl<FX> TryIntoIdentMap<Expr<FX>> for Expr<FX> {
    fn try_into_identmap(&self) -> Option<&IdentMap<Expr<FX>>> {
        match self {
            Expr::Object(od) => od.try_into_identmap(),
            _ => None,
        }
    }
}

impl<FX> Unparse for Expr<FX>
where
    FX: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use Expr::*;

        match self {
            Lit(x) => x.unparse_into(s),
            Ref(x) => x.unparse_into(s),
            Object(x) => x.unparse_into(s),
            Let(x) => x.unparse_into(s),
            Match(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
            Lookup(x) => x.unparse_into(s),
            Effect(x) => x.unparse_into(s),
        }
    }
}

impl<FX> fmt::Display for Expr<FX>
where
    FX: Unparse,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}
