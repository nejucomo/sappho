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

use derive_enum_from_into::{EnumFrom, EnumTryInto};
use derive_more::{From, Into};
use derive_new::new;
use sappho_ast_effect::{ProcEffect, PureEffect, QueryEffect};
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
#[derive(Debug, From, Into)]
#[from(Wise<FX>)]
pub struct BoxWise<FX>(Box<Wise<FX>>);

/// **Wi**th **S**ource **E**xpression
#[derive(Debug, From, Into)]
pub struct Wise<FX>(WithSource<Expr<FX>>);

// Generic structures across effects:
#[derive(Debug, EnumFrom, EnumTryInto)]
pub enum Expr<FX> {
    Func(FuncDef),
    Query(QueryDef),
    Proc(ProcDef),
    Let(Let<FX>),
    Match(Match<FX>),
    Applications(Applications<FX>),
}

#[derive(Debug, new)]
pub struct FuncDef {
    argpat: Pattern,
    #[new(into)]
    body: PureExpr,
}

#[derive(Clone, Debug, PartialEq, EnumFrom, EnumTryInto)]
pub enum Pattern {
    Bind(BindPattern),
    LitEq(PrimVal),
    Unpack(Attrs<Pattern>),
    List(ListForm<Pattern, BindPattern>),
}

#[derive(Clone, Debug, PartialEq, From)]
pub struct BindPattern(RcId);

#[derive(Debug, From)]
pub struct QueryDef(QueryExpr);

#[derive(Debug, From)]
pub struct ProcDef(ProcExpr);

#[derive(Debug, new)]
pub struct Let<FX> {
    clauses: Vec<LetClause<FX>>,
    #[new(into)]
    inner: BoxWise<FX>,
}

#[derive(Debug, new)]
pub struct LetClause<FX> {
    binding: Pattern,
    #[new(into)]
    definition: BoxWise<FX>,
}

#[derive(Debug, new)]
pub struct Match<FX> {
    #[new(into)]
    candidate: BoxWise<FX>,
    clauses: Vec<MatchClause<FX>>,
}

#[derive(Debug, new)]
pub struct MatchClause<FX> {
    binding: Pattern,
    #[new(into)]
    consequent: BoxWise<FX>,
}

#[derive(Debug, From)]
pub struct Applications<FX>(LeftAssoc<Lookups<FX>, Application<FX>>);

#[derive(Debug, From)]
pub struct Application<FX>(Lookups<FX>);

#[derive(Debug, From)]
pub struct Lookups<FX>(LeftAssoc<Interactions<FX>, Lookup>);

#[derive(Debug, From)]
pub struct Lookup(RcId);

// Potentially effectful expressions:
#[derive(Debug, From, new)]
pub struct Interactions<FX> {
    pub effects: Vec<FX>,
    pub confined: Confined<FX>,
}

#[derive(Debug, EnumFrom, EnumTryInto)]
pub enum Confined<FX> {
    Ref(RcId),
    Prim(PrimVal),
    Parens(ParensExpr<FX>),
    ObjectDef(Object<FuncDef, QueryDef, ProcDef, Wise<FX>>),
    ListExpr(ListForm<Wise<FX>, BoxWise<FX>>),
}

#[derive(Debug, From)]
pub struct ParensExpr<FX>(BoxWise<FX>);

#[cfg(test)]
mod tests;

#[cfg(test)]
mod testeq;
