use either::Either::{self, Left};
use sappho_ast_core::{AstProvider, CoreExpr, FuncDef, ObjectDef, ProcDef, QueryDef};
use sappho_ast_effect::Effect;
use sappho_ast_reduced as astred;
use sappho_ast_rich as rich;

use crate::xform::listimpls::TailOrAttrs;
use crate::xform::{TransformInto, TryTransformInto};

impl<FX> TransformInto<astred::Expr<FX>> for rich::Expr<FX>
where
    FX: Effect,
{
    fn transform(self) -> astred::Expr<FX> {
        match self {
            rich::Expr::Core(x) => astred::Expr::new(x.transform()),
            rich::Expr::Func(x) => astred::Expr::new(x.transform()),
            rich::Expr::Query(x) => astred::Expr::new(x.transform()),
            rich::Expr::Proc(x) => astred::Expr::new(x.transform()),
            rich::Expr::List(x) => x.transform(),
        }
    }
}

impl<FX> TransformInto<rich::Expr<FX>> for astred::Expr<FX>
where
    FX: Effect,
{
    fn transform(self) -> rich::Expr<FX> {
        use sappho_object::Unbundled::*;
        use CoreExpr::*;

        let cx: CoreExpr<_, _> = self.into();
        // rich::Expr::Core(cx.transform())
        match cx {
            Object(obj) => match obj.unbundle() {
                Bundled(obj) => ObjectDef::new(obj.transform()).into(),
                Func(f) => rich::Expr::Func(f.transform()),
                Query(q) => rich::Expr::Query(q.transform()),
                Proc(p) => rich::Expr::Proc(p.transform()),
                Attrs(attrs) => attrs
                    .try_transform()
                    .map_left(rich::Expr::List)
                    .map_right(|attrs| rich::Expr::from(attrs.transform()))
                    .into_inner(),
            },

            core => rich::Expr::Core(core.transform()),
        }
    }
}

impl<FX> TryTransformInto<TailOrAttrs<Box<rich::Expr<FX>>, astred::Expr<FX>>> for astred::Expr<FX>
where
    FX: Effect,
{
    fn try_transform(self) -> Either<TailOrAttrs<Box<rich::Expr<FX>>, astred::Expr<FX>>, Self> {
        use CoreExpr::*;
        use TailOrAttrs::*;

        match CoreExpr::from(self) {
            Object(obj) => obj
                .try_transform()
                .map_left(TailAttrs)
                .map_right(|obj| astred::Expr::from(Object(obj))),
            other => Left(Tail(Box::new(rich::Expr::Core(other.transform())))),
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
