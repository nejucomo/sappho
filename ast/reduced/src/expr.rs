use std::fmt;
use std::ops::Deref;

use sappho_ast as ast;
use sappho_ast_effect::Effect;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

use crate::{CoreExpr, ObjectDef};

#[derive(Clone, Debug, PartialEq)]
pub struct Expr<FX>(CoreExpr<FX>)
where
    FX: Effect;

impl<FX> Expr<FX>
where
    FX: Effect,
{
    pub fn new<T>(x: T) -> Self
    where
        CoreExpr<FX>: From<T>,
    {
        Expr(CoreExpr::from(x))
    }
}

impl<FX> Deref for Expr<FX>
where
    FX: Effect,
{
    type Target = CoreExpr<FX>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<FX> From<ast::Expr<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(x: ast::Expr<FX>) -> Self {
        match x {
            ast::Expr::Core(x) => Expr(x.transform_into()),
            ast::Expr::Func(x) => {
                Expr(ast::CoreExpr::from(ast::ObjectDef::new_func(x)).transform_into())
            }
            ast::Expr::Query(x) => {
                Expr(ast::CoreExpr::from(ast::ObjectDef::new_query(x)).transform_into())
            }
            ast::Expr::Proc(x) => {
                Expr(ast::CoreExpr::from(ast::ObjectDef::new_proc(x)).transform_into())
            }
            ast::Expr::List(x) => x.into(),
        }
    }
}

impl<FX> From<ast::ListExpr<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(x: ast::ListExpr<FX>) -> Self {
        use sappho_ast_core::CoreExpr::Object;

        x.into_reverse_fold(
            |opttail| {
                opttail
                    .map(|x| Expr::from(*x))
                    .unwrap_or_else(|| Expr(Object(ObjectDef::default())))
            },
            |tail, head| {
                Expr(Object(ObjectDef::new_attrs([
                    ("head".to_string(), Expr::from(head)),
                    ("tail".to_string(), tail),
                ])))
            },
        )
    }
}

impl<FX> From<Expr<FX>> for ast::Expr<FX>
where
    FX: Effect,
{
    fn from(x: Expr<FX>) -> Self {
        match x.0 {
            CoreExpr::Object(obj) => objdef_to_ast_expr(obj),
            core => ast::Expr::Core(core.transform_into()),
        }
    }
}

fn objdef_to_ast_expr<FX>(objdef: ObjectDef<FX>) -> ast::Expr<FX>
where
    FX: Effect,
{
    use ast::Expr::{Core, Func, List, Proc, Query};
    use sappho_ast_core::CoreExpr::Object;
    use sappho_object::Unbundled as U;

    match objdef.unbundle() {
        U::Bundled(obj) => Core(Object(ObjectDef::new(obj).transform_into())),
        U::Func(f) => Func(f.transform_into()),
        U::Query(q) => Query(q.transform_into()),
        U::Proc(p) => Proc(p.transform_into()),
        U::Attrs(a) => a
            .as_list_form()
            .map(|listform| {
                List(
                    listform
                        .map_elems(|x| ast::Expr::from(x.clone()))
                        .map_tail(|x| Box::new(ast::Expr::from(x.clone()))),
                )
            })
            .unwrap_or_else(|| Core(Object(ObjectDef::new_attrs(a).transform_into()))),
    }
}

impl<FX> TryIntoIdentMap<Expr<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&IdentMap<Expr<FX>>> {
        self.0.try_into_identmap()
    }
}

impl<FX> Unparse for Expr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s);
    }
}

impl<FX> fmt::Display for Expr<FX>
where
    FX: Effect,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}
