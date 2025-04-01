use sappho_effect::Effect;
use sappho_kast::{
    BoxWise, FuncDef, Let, LetClause, Match, MatchClause, ObjectDef, ProcDef, QueryDef, Wise,
};
use sappho_listform::ListForm;
use sappho_object::Object;
use sappho_syntax::{self as syntax, SyntaxProvider};
use sappho_with_source::WithSource;

use crate::{self as east, EastProvider};

pub(crate) struct FromHelper<T>(T);

pub(crate) fn from_unwrap<S, T>(src: S) -> T
where
    FromHelper<T>: From<S>,
{
    FromHelper::from(src).0
}

impl<T> FromHelper<T> {
    fn map<F, U>(self, f: F) -> FromHelper<U>
    where
        F: FnOnce(T) -> U,
    {
        FromHelper(f(self.0))
    }
}

impl<FX> From<BoxWise<SyntaxProvider, FX>> for FromHelper<BoxWise<EastProvider, FX>>
where
    FX: Effect,
{
    fn from(syn: BoxWise<SyntaxProvider, FX>) -> Self {
        FromHelper::from(syn.unwrap()).map(BoxWise::from)
    }
}

impl<FX> From<Wise<SyntaxProvider, FX>> for FromHelper<Wise<EastProvider, FX>>
where
    FX: Effect,
{
    fn from(syn: Wise<SyntaxProvider, FX>) -> Self {
        FromHelper::from(syn.unwrap()).map(Wise::from)
    }
}

impl<FX> From<WithSource<syntax::Expr<FX>>> for FromHelper<WithSource<east::Expr<FX>>>
where
    FX: Effect,
{
    fn from(syn: WithSource<syntax::Expr<FX>>) -> Self {
        FromHelper(syn.map(east::Expr::from))
    }
}

impl<FX> From<Let<SyntaxProvider, FX>> for FromHelper<Let<EastProvider, FX>>
where
    FX: Effect,
{
    fn from(syn: Let<SyntaxProvider, FX>) -> Self {
        FromHelper(Let::new(from_unwrap(syn.clauses), from_unwrap(syn.inner)))
    }
}

impl<FX> From<LetClause<SyntaxProvider, FX>> for FromHelper<LetClause<EastProvider, FX>>
where
    FX: Effect,
{
    fn from(syn: LetClause<SyntaxProvider, FX>) -> Self {
        FromHelper(LetClause::new(syn.binding, from_unwrap(syn.definition)))
    }
}

impl<FX> From<Match<SyntaxProvider, FX>> for FromHelper<Match<EastProvider, FX>>
where
    FX: Effect,
{
    fn from(syn: Match<SyntaxProvider, FX>) -> Self {
        FromHelper(Match::new(
            from_unwrap(syn.candidate),
            from_unwrap(syn.clauses),
        ))
    }
}

impl<FX> From<MatchClause<SyntaxProvider, FX>> for FromHelper<MatchClause<EastProvider, FX>>
where
    FX: Effect,
{
    fn from(syn: MatchClause<SyntaxProvider, FX>) -> Self {
        FromHelper(MatchClause::new(syn.binding, from_unwrap(syn.consequent)))
    }
}

impl From<FuncDef<SyntaxProvider>> for FromHelper<FuncDef<EastProvider>> {
    fn from(syn: FuncDef<SyntaxProvider>) -> Self {
        FromHelper(FuncDef::new(syn.argpat, from_unwrap(syn.body)))
    }
}

impl From<QueryDef<SyntaxProvider>> for FromHelper<QueryDef<EastProvider>> {
    fn from(syn: QueryDef<SyntaxProvider>) -> Self {
        FromHelper(QueryDef::new(from_unwrap(syn.unwrap())))
    }
}

impl From<ProcDef<SyntaxProvider>> for FromHelper<ProcDef<EastProvider>> {
    fn from(syn: ProcDef<SyntaxProvider>) -> Self {
        FromHelper(ProcDef::new(from_unwrap(syn.unwrap())))
    }
}

impl<FX> From<ObjectDef<SyntaxProvider, FX>> for FromHelper<ObjectDef<EastProvider, FX>>
where
    FX: Effect,
{
    fn from(syn: ObjectDef<SyntaxProvider, FX>) -> Self {
        FromHelper(ObjectDef::from(from_unwrap(syn.unwrap())))
    }
}

impl<FX>
    From<
        Object<
            FuncDef<SyntaxProvider>,
            QueryDef<SyntaxProvider>,
            ProcDef<SyntaxProvider>,
            Wise<SyntaxProvider, FX>,
        >,
    >
    for FromHelper<
        Object<
            FuncDef<EastProvider>,
            QueryDef<EastProvider>,
            ProcDef<EastProvider>,
            Wise<EastProvider, FX>,
        >,
    >
where
    FX: Effect,
{
    fn from(
        value: Object<
            FuncDef<SyntaxProvider>,
            QueryDef<SyntaxProvider>,
            ProcDef<SyntaxProvider>,
            Wise<SyntaxProvider, FX>,
        >,
    ) -> Self {
        FromHelper(value.map_parts(from_unwrap, from_unwrap, from_unwrap, from_unwrap))
    }
}

impl<FX> From<ListForm<Wise<SyntaxProvider, FX>, BoxWise<SyntaxProvider, FX>>>
    for FromHelper<ListForm<Wise<EastProvider, FX>, BoxWise<EastProvider, FX>>>
where
    FX: Effect,
{
    fn from(value: ListForm<Wise<SyntaxProvider, FX>, BoxWise<SyntaxProvider, FX>>) -> Self {
        FromHelper(
            value
                .into_iter()
                .map(|ei| ei.map_left(from_unwrap).map_right(from_unwrap))
                .collect(),
        )
    }
}

impl<S, T> From<Vec<S>> for FromHelper<Vec<T>>
where
    FromHelper<T>: From<S>,
{
    fn from(src: Vec<S>) -> Self {
        FromHelper(src.into_iter().map(from_unwrap).collect())
    }
}
