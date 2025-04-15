use sappho_effect::Effect;
use sappho_kast::ObjectDefInner;
use sappho_listform::ListForm;
use sappho_syntax::{self as syntax, SyntaxProvider};

use crate::transform::TransformInto;
use crate::{EastProvider, FuncDef, ObjectDef, ProcDef, QueryDef};

impl<FX> TransformInto<ObjectDef<FX>> for syntax::ObjectDef<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> ObjectDef<FX> {
        ObjectDef::from(self.unwrap().transform_into())
    }
}

impl<FX> TransformInto<ObjectDefInner<EastProvider, FX>> for ObjectDefInner<SyntaxProvider, FX>
where
    FX: Effect,
{
    fn transform_into(self) -> ObjectDefInner<EastProvider, FX> {
        self.map_parts(
            |f| f.transform_into(),
            |q| q.transform_into(),
            |p| p.transform_into(),
            |a| a.transform_into(),
        )
    }
}

impl TransformInto<FuncDef> for syntax::FuncDef {
    fn transform_into(self) -> FuncDef {
        FuncDef::new(self.argpat, self.body.transform_into())
    }
}

impl TransformInto<QueryDef> for syntax::QueryDef {
    fn transform_into(self) -> QueryDef {
        QueryDef::new(self.unwrap().transform_into())
    }
}

impl TransformInto<ProcDef> for syntax::ProcDef {
    fn transform_into(self) -> ProcDef {
        ProcDef::new(self.unwrap().transform_into())
    }
}

impl<XS, XT, TS, TT> TransformInto<ListForm<XT, TT>> for ListForm<XS, TS>
where
    XS: TransformInto<XT>,
    TS: TransformInto<TT>,
    XT: std::fmt::Debug,
    TT: std::fmt::Debug,
{
    fn transform_into(self) -> ListForm<XT, TT> {
        self.into_iter()
            .map(|ei| {
                ei.map_left(XS::transform_into)
                    .map_right(TS::transform_into)
            })
            .collect()
    }
}
