use crate::{FuncDef, QueryDef};
use sappho_object::Object;

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
pub type ObjectDef<Pattern, PureExpr, QueryExpr, Expr> =
    Object<FuncDef<Pattern, PureExpr>, QueryDef<QueryExpr>, Expr>;

pub fn transform_object_def<P, PD, X, XD, Q, QD, G, GD>(
    obj: ObjectDef<P, X, Q, G>,
) -> ObjectDef<PD, XD, QD, GD>
where
    PD: From<P>,
    XD: From<X>,
    QD: From<Q>,
    GD: From<G>,
{
    obj.transform(
        |func| func.transform_into(),
        |query| query.transform_into(),
        GD::from,
    )
}
