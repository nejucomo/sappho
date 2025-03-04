use either::Either::{self, Left, Right};
use sappho_ast::{self as ast, Ast};
use sappho_ast_core::{AstProvider, CmtExpr, CoreExpr, FuncDef, ObjectDef, ProcDef, QueryDef};
use sappho_ast_effect::Effect;
use sappho_ast_reduced::{self as astred, AstRed};

use crate::xform::listimpls::TailOrAttrs;
use crate::xform::{TransformInto, TryTransformInto};

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
            ast::Expr::List(x) => {
                let cx: CmtExpr<AstRed, FX> = x.transform();
                let (optcmt, inner) = cx.into();
                assert!(optcmt.is_none(), "{optcmt:?}");
                inner
            }
        }
    }
}

impl<FX> TransformInto<ast::Expr<FX>> for astred::Expr<FX>
where
    FX: Effect,
{
    fn transform(self) -> ast::Expr<FX> {
        use sappho_object::Unbundled::*;
        use CoreExpr::*;

        let cx: CoreExpr<_, _> = self.into();
        match cx {
            Object(obj) => match obj.unbundle() {
                Bundled(obj) => ObjectDef::new(obj.transform()).into(),
                Func(f) => ast::Expr::Func(f.transform()),
                Query(q) => ast::Expr::Query(q.transform()),
                Proc(p) => ast::Expr::Proc(p.transform()),
                Attrs(attrs) => attrs
                    .try_transform()
                    .map_left(ast::Expr::List)
                    .map_right(|attrs| ast::Expr::from(attrs.transform()))
                    .into_inner(),
            },

            core => ast::Expr::Core(core.transform()),
        }
    }
}

impl<FX> TryTransformInto<TailOrAttrs<CmtExpr<Ast, FX>, CmtExpr<AstRed, FX>>>
    for CmtExpr<AstRed, FX>
where
    FX: Effect,
{
    fn try_transform(self) -> Either<TailOrAttrs<CmtExpr<Ast, FX>, CmtExpr<AstRed, FX>>, Self> {
        use TailOrAttrs::*;

        let (optcmt, expr) = self.into();

        match expr.try_transform() {
            Left(toa) => Left(match toa {
                Tail(t) => Tail((optcmt, t).into()),
                TailAttrs(attrs) => TailAttrs(attrs),
            }),
            Right(expr) => Right((optcmt, expr).into()),
        }
    }
}

impl<FX> TryTransformInto<TailOrAttrs<ast::Expr<FX>, CmtExpr<AstRed, FX>>> for astred::Expr<FX>
where
    FX: Effect,
{
    fn try_transform(self) -> Either<TailOrAttrs<ast::Expr<FX>, CmtExpr<AstRed, FX>>, Self> {
        use CoreExpr::*;
        use TailOrAttrs::*;

        match CoreExpr::from(self) {
            Object(obj) => obj
                .try_transform()
                .map_left(TailAttrs)
                .map_right(|obj| astred::Expr::from(Object(obj))),
            other => Left(Tail(ast::Expr::Core(other.transform()))),
        }
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
