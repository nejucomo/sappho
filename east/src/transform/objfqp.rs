use sappho_effect::Effect;
use sappho_listform::ListForm;
use sappho_object::Object;
use sappho_syntax as syntax;

use crate::transform::TransformInto;
use crate::{FuncDef, ObjectDef, ProcDef, QueryDef, Wise};

impl<FX> TransformInto<ObjectDef<FX>> for syntax::ObjectDef<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> ObjectDef<FX> {
        ObjectDef::from(self.unwrap().transform_into())
    }
}

impl<FX> TransformInto<Object<FuncDef, QueryDef, ProcDef, Wise<FX>>>
    for Object<syntax::FuncDef, syntax::QueryDef, syntax::ProcDef, syntax::Wise<FX>>
where
    FX: Effect,
{
    fn transform_into(self) -> Object<FuncDef, QueryDef, ProcDef, Wise<FX>> {
        self.map_parts(
            syntax::FuncDef::transform_into,
            syntax::QueryDef::transform_into,
            syntax::ProcDef::transform_into,
            syntax::Wise::transform_into,
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
