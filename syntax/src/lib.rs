/// # Todo
///
/// Move `ListForm` into this crate.
pub mod leftassoc;

mod applications;
mod boxwise;
mod confined;
mod expr;
mod funcdef;
mod interactions;
mod letexpr;
mod lookups;
mod matchexpr;
mod parens;
mod parseparams;
mod parserext;
mod pattern;
mod procdef;
mod querydef;
mod wise;

use derive_more::{From, Into, TryInto};
use derive_new::new;
use sappho_ast_effect::{Effect, ProcEffect, PureEffect, QueryEffect};
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_object::Object;
use sappho_primval::PrimVal;
use sappho_with_source::WithSource;

use crate::leftassoc::LeftAssoc;

// Top-level expressions for each effect kind:
pub type PureExpr = BoxWise<PureEffect>;
pub type QueryExpr = BoxWise<QueryEffect>;
pub type ProcExpr = BoxWise<ProcEffect>;

// Top-Level Recursion Nexus

/// Boxed-Spanned-Expression
#[derive(Debug, PartialEq, From, Into)]
#[from(Wise<FX>)]
pub struct BoxWise<FX>(Box<Wise<FX>>)
where
    FX: Effect;

/// **Wi**th **S**ource **E**xpression
#[derive(Debug, From, Into)]
pub struct Wise<FX>(WithSource<Expr<FX>>)
where
    FX: Effect;

// Generic structures across effects:
#[derive(Debug, PartialEq, From, TryInto)]
pub enum Expr<FX>
where
    FX: Effect,
{
    Func(FuncDef),
    Query(QueryDef),
    Proc(ProcDef),
    Let(Let<FX>),
    Match(Match<FX>),
    Applications(Applications<FX>),
}

#[derive(Debug, PartialEq, new)]
pub struct FuncDef {
    argpat: Pattern,
    #[new(into)]
    body: PureExpr,
}

#[derive(Clone, Debug, PartialEq, From, TryInto)]
pub enum Pattern {
    Bind(BindPattern),
    LitEq(PrimVal),
    Unpack(Attrs<Pattern>),
    List(ListForm<Pattern, BindPattern>),
}

#[derive(Clone, Debug, PartialEq, From)]
pub struct BindPattern(RcId);

#[derive(Debug, PartialEq, From)]
pub struct QueryDef(QueryExpr);

#[derive(Debug, PartialEq, From)]
pub struct ProcDef(ProcExpr);

#[derive(Debug, PartialEq, new)]
pub struct Let<FX>
where
    FX: Effect,
{
    clauses: Vec<LetClause<FX>>,
    #[new(into)]
    inner: BoxWise<FX>,
}

#[derive(Debug, PartialEq, new)]
pub struct LetClause<FX>
where
    FX: Effect,
{
    binding: Pattern,
    #[new(into)]
    definition: BoxWise<FX>,
}

#[derive(Debug, PartialEq, new)]
pub struct Match<FX>
where
    FX: Effect,
{
    #[new(into)]
    candidate: BoxWise<FX>,
    clauses: Vec<MatchClause<FX>>,
}

#[derive(Debug, PartialEq, new)]
pub struct MatchClause<FX>
where
    FX: Effect,
{
    binding: Pattern,
    #[new(into)]
    consequent: BoxWise<FX>,
}

#[derive(Debug, PartialEq, From)]
pub struct Applications<FX>(LeftAssoc<Lookups<FX>, Application<FX>>)
where
    FX: Effect;

#[derive(Debug, PartialEq, From)]
pub struct Application<FX>(Lookups<FX>)
where
    FX: Effect;

#[derive(Debug, PartialEq, From)]
pub struct Lookups<FX>(LeftAssoc<Interactions<FX>, Lookup>)
where
    FX: Effect;

#[derive(Debug, PartialEq, From)]
pub struct Lookup(RcId);

// Potentially effectful expressions:
#[derive(Debug, PartialEq, From, new)]
pub struct Interactions<FX>
where
    FX: Effect,
{
    pub effects: Vec<FX>,
    pub confined: Confined<FX>,
}

#[derive(Debug, PartialEq, From, TryInto)]
pub enum Confined<FX>
where
    FX: Effect,
{
    Ref(RcId),
    Prim(PrimVal),
    Parens(ParensExpr<FX>),
    ObjectDef(Object<FuncDef, QueryDef, ProcDef, Wise<FX>>),
    ListExpr(ListForm<Wise<FX>, BoxWise<FX>>),
}

#[derive(Debug, PartialEq, From)]
pub struct ParensExpr<FX>(BoxWise<FX>)
where
    FX: Effect;

#[cfg(test)]
mod tests;
