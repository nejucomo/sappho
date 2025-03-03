use either::Either::{self, Left, Right};
use sappho_ast_core::{AstProvider, FuncDef, ObjectDef, ProcDef, QueryDef};
use sappho_ast_effect::Effect;
use sappho_attrs::Attrs;
use sappho_object::{Object, Unbundled};

use crate::xform::{TransformInto, TryTransformInto};

impl<XPS, XPD, FX> TransformInto<ObjectDef<XPD, FX>> for ObjectDef<XPS, FX>
where
    XPS: AstProvider,
    XPD: AstProvider,
    FX: Effect,
    XPS::Expr<FX>: TransformInto<XPD::Expr<FX>>,
    FuncDef<XPS>: TransformInto<FuncDef<XPD>>,
    QueryDef<XPS>: TransformInto<QueryDef<XPD>>,
    ProcDef<XPS>: TransformInto<ProcDef<XPD>>,
{
    fn transform(self) -> ObjectDef<XPD, FX> {
        ObjectDef::from(Object::from(self).transform())
    }
}

impl<SF, SQ, SP, SA, DF, DQ, DP, DA> TransformInto<Object<DF, DQ, DP, DA>>
    for Object<SF, SQ, SP, SA>
where
    SF: TransformInto<DF>,
    SQ: TransformInto<DQ>,
    SP: TransformInto<DP>,
    SA: TransformInto<DA>,
{
    fn transform(self) -> Object<DF, DQ, DP, DA> {
        self.map_parts(SF::transform, SQ::transform, SP::transform, SA::transform)
    }
}

impl<XP, FX> TryTransformInto<Attrs<XP::Expr<FX>>> for ObjectDef<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn try_transform(self) -> Either<Attrs<XP::Expr<FX>>, Self> {
        Object::from(self)
            .try_transform()
            .map_right(ObjectDef::from)
    }
}

impl<F, Q, P, A> TryTransformInto<Attrs<A>> for Object<F, Q, P, A> {
    fn try_transform(self) -> Either<Attrs<A>, Self> {
        self.unbundle().try_transform().map_right(Object::from)
    }
}

impl<F, Q, P, A> TryTransformInto<Attrs<A>> for Unbundled<F, Q, P, A> {
    fn try_transform(self) -> Either<Attrs<A>, Self> {
        match self {
            Unbundled::Attrs(attrs) => Left(attrs),
            other => Right(other),
        }
    }
}
