use crate::{
    ApplicationExpr, AstFxFor, FromFx, Identifier, LetExpr, Literal, LookupExpr, MatchExpr,
    ObjectDef,
};
use sappho_ast as ast;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

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

impl<FX> From<ast::ListExpr<AstFxFor<FX>>> for GenExpr<FX>
where
    FX: FromFx,
{
    fn from(x: ast::ListExpr<AstFxFor<FX>>) -> Self {
        use GenExpr::Object;

        x.into_reverse_fold(
            |opttail| {
                opttail
                    .map(|x| GenExpr::from(*x))
                    .unwrap_or_else(|| Object(ObjectDef::default()))
            },
            |tail, head| {
                Object(ObjectDef::new_attrs([
                    ("head".to_string(), GenExpr::from(head)),
                    ("tail".to_string(), tail),
                ]))
            },
        )
    }
}

impl<FX> From<GenExpr<AstFxFor<FX>>> for ast::GenExpr<FX>
where
    FX: FromFx,
    AstFxFor<FX>: Clone,
{
    fn from(x: GenExpr<AstFxFor<FX>>) -> Self {
        use GenExpr::*;

        match x {
            Lit(x) => ast::GenExpr::Lit(x),
            Ref(x) => ast::GenExpr::Ref(x),
            Object(x) => objdef_to_ast_expr(x),
            Let(x) => ast::GenExpr::Let(x.transform_into()),
            Match(x) => ast::GenExpr::Match(x.transform_into()),
            Application(x) => ast::GenExpr::Application(x.transform_into()),
            Lookup(x) => ast::GenExpr::Lookup(x.transform_into()),
            Effect(x) => ast::GenExpr::Effect(FX::from_fx(x)),
        }
    }
}

fn objdef_to_ast_expr<FX>(objdef: ObjectDef<AstFxFor<FX>>) -> ast::GenExpr<FX>
where
    FX: FromFx,
    AstFxFor<FX>: Clone,
{
    use ast::GenExpr::{Func, List, Object, Query};
    use sappho_gast::Unbundled as U;

    match objdef.unbundle() {
        U::Bundled(obj) => Object(obj.transform_into()),
        U::Func(f) => Func(f.transform_into()),
        U::Query(q) => Query(q.transform_into()),
        U::Attrs(a) => a
            .as_list_form()
            .map(|listform| {
                List(
                    listform
                        .map_elems(|x| ast::GenExpr::from(x.clone()))
                        .map_tail(|x| Box::new(ast::GenExpr::from(x.clone()))),
                )
            })
            .unwrap_or_else(|| Object(ObjectDef::new_attrs(a).transform_into())),
    }
}

impl<FX> TryIntoIdentMap<GenExpr<FX>> for GenExpr<FX> {
    fn try_into_identmap(&self) -> Option<&IdentMap<GenExpr<FX>>> {
        match self {
            GenExpr::Object(od) => od.try_into_identmap(),
            _ => None,
        }
    }
}

impl<FX> Unparse for GenExpr<FX>
where
    FX: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use GenExpr::*;

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

impl<FX> std::fmt::Display for GenExpr<FX>
where
    FX: Unparse,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.unparse(f, 0)
    }
}
