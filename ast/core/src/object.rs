use crate::{FuncDef, ProcDef, QueryDef};
use sappho_object::Object;

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
pub type ObjectDef<Pattern, PureExpr, QueryExpr, ProcExpr, Expr> =
    Object<FuncDef<Pattern, PureExpr>, QueryDef<QueryExpr>, ProcDef<ProcExpr>, Expr>;

pub fn transform_object_def<
    Pat,
    Pure,
    Query,
    Proc,
    Generic,
    DstPat,
    DstPure,
    DstQuery,
    DstProc,
    DstGeneric,
>(
    obj: ObjectDef<Pat, Pure, Query, Proc, Generic>,
) -> ObjectDef<DstPat, DstPure, DstQuery, DstProc, DstGeneric>
where
    DstPat: From<Pat>,
    DstPure: From<Pure>,
    DstQuery: From<Query>,
    DstProc: From<Proc>,
    DstGeneric: From<Generic>,
{
    obj.transform(
        |func| func.transform_into(),
        |query| query.transform_into(),
        |proc| proc.transform_into(),
        DstGeneric::from,
    )
}
