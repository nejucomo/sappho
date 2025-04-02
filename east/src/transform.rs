use sappho_effect::{Effect, ProcEffect, PureEffect, QueryEffect};
use sappho_kast::{
    BoxWise, FuncDef, KastProvider, Let, LetClause, Match, MatchClause, ObjectDef, ProcDef,
    QueryDef, Wise,
};
use sappho_listform::ListForm;
use sappho_object::Object;
use sappho_with_source::WithSource;

pub(crate) trait TransformInto<T> {
    fn transform_into(self) -> T;
}

impl<S, T> TransformInto<Vec<T>> for Vec<S>
where
    S: TransformInto<T>,
{
    fn transform_into(self) -> Vec<T> {
        self.into_iter().map(S::transform_into).collect()
    }
}

impl<KS, KT, FX> TransformInto<BoxWise<KT, FX>> for BoxWise<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformInto<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> BoxWise<KT, FX> {
        BoxWise::from(self.unwrap().transform_into())
    }
}

impl<KS, KT> TransformInto<FuncDef<KT>> for FuncDef<KS>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<PureEffect>: TransformInto<KT::Expr<PureEffect>>,
{
    fn transform_into(self) -> FuncDef<KT> {
        FuncDef::new(self.argpat, self.body.transform_into())
    }
}

impl<KS, KT, FX> TransformInto<LetClause<KT, FX>> for LetClause<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformInto<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> LetClause<KT, FX> {
        LetClause::new(self.binding, self.definition.transform_into())
    }
}

impl<KS, KT, FX> TransformInto<Let<KT, FX>> for Let<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformInto<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> Let<KT, FX> {
        Let::new(self.clauses.transform_into(), self.inner.transform_into())
    }
}

impl<KS, KT, FX> TransformInto<MatchClause<KT, FX>> for MatchClause<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformInto<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> MatchClause<KT, FX> {
        MatchClause::new(self.binding, self.consequent.transform_into())
    }
}

impl<KS, KT, FX> TransformInto<Match<KT, FX>> for Match<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformInto<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> Match<KT, FX> {
        Match::new(
            self.candidate.transform_into(),
            self.clauses.transform_into(),
        )
    }
}

impl<KS, KT, FX> TransformInto<ObjectDef<KT, FX>> for ObjectDef<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<PureEffect>: TransformInto<KT::Expr<PureEffect>>,
    KS::Expr<QueryEffect>: TransformInto<KT::Expr<QueryEffect>>,
    KS::Expr<ProcEffect>: TransformInto<KT::Expr<ProcEffect>>,
    KS::Expr<FX>: TransformInto<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> ObjectDef<KT, FX> {
        ObjectDef::from(self.unwrap().transform_into())
    }
}

impl<KS, KT, FX> TransformInto<Object<FuncDef<KT>, QueryDef<KT>, ProcDef<KT>, Wise<KT, FX>>>
    for Object<FuncDef<KS>, QueryDef<KS>, ProcDef<KS>, Wise<KS, FX>>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<PureEffect>: TransformInto<KT::Expr<PureEffect>>,
    KS::Expr<QueryEffect>: TransformInto<KT::Expr<QueryEffect>>,
    KS::Expr<ProcEffect>: TransformInto<KT::Expr<ProcEffect>>,
    KS::Expr<FX>: TransformInto<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> Object<FuncDef<KT>, QueryDef<KT>, ProcDef<KT>, Wise<KT, FX>> {
        self.map_parts(
            FuncDef::<KS>::transform_into,
            QueryDef::<KS>::transform_into,
            ProcDef::<KS>::transform_into,
            Wise::<KS, FX>::transform_into,
        )
    }
}

impl<KS, KT> TransformInto<ProcDef<KT>> for ProcDef<KS>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<ProcEffect>: TransformInto<KT::Expr<ProcEffect>>,
{
    fn transform_into(self) -> ProcDef<KT> {
        ProcDef::new(self.unwrap().transform_into())
    }
}

impl<KS, KT> TransformInto<QueryDef<KT>> for QueryDef<KS>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<QueryEffect>: TransformInto<KT::Expr<QueryEffect>>,
{
    fn transform_into(self) -> QueryDef<KT> {
        QueryDef::new(self.unwrap().transform_into())
    }
}

impl<KS, KT, FX> TransformInto<Wise<KT, FX>> for Wise<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformInto<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> Wise<KT, FX> {
        Wise::from(self.unwrap().transform_into())
    }
}

impl<S, T> TransformInto<WithSource<T>> for WithSource<S>
where
    S: TransformInto<T>,
{
    fn transform_into(self) -> WithSource<T> {
        self.map(S::transform_into)
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
