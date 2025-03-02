use sappho_ast::{self as ast};
use sappho_ast_core::{AstProvider, CoreExpr, FuncDef, ProcDef, QueryDef};
use sappho_ast_effect::Effect;
use sappho_ast_reduced::{self as astred};

use crate::xform::TransformInto;

impl<FX> TransformInto<astred::Expr<FX>> for ast::Expr<FX>
where
    FX: Effect,
{
    fn transform(self) -> astred::Expr<FX> {
        match self {
            ast::Expr::Core(x) => astred::Expr::new(x.transform()),
            ast::Expr::Func(x) => astred::Expr::new(x.transform()),
            ast::Expr::Query(x) => astred::Expr::new(x.transform()),
            ast::Expr::Proc(x) => astred::Expr::new(x.transform()),
            ast::Expr::List(x) => x.transform(),
        }
    }
}

impl<FX> TransformInto<ast::Expr<FX>> for astred::Expr<FX>
where
    FX: Effect,
{
    fn transform(self) -> ast::Expr<FX> {
        let cx: CoreExpr<_, _> = self.into();
        ast::Expr::Core(cx.transform())
    }
}

impl<XPS, XPD, FX> TransformInto<CoreExpr<XPD, FX>> for CoreExpr<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
    XPS::Pattern: TransformInto<XPD::Pattern>,
    FuncDef<XPS>: TransformInto<FuncDef<XPD>>,
    QueryDef<XPS>: TransformInto<QueryDef<XPD>>,
    ProcDef<XPS>: TransformInto<ProcDef<XPD>>,
{
    fn transform(self) -> CoreExpr<XPD, FX> {
        use CoreExpr::*;

        match self {
            Lit(x) => Lit(x),
            Ref(x) => Ref(x),
            Object(x) => Object(x.transform()),
            Let(x) => Let(x.transform()),
            Match(x) => Match(x.transform()),
            Application(x) => Application(x.transform()),
            Lookup(x) => Lookup(x.transform()),
            Effect(x) => Effect(x.transform()),
        }
    }
}
