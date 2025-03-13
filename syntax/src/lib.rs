/// # Todo
///
/// Move `ListForm` into this crate.
pub mod leftassoc;
pub mod spanned;

mod applications;
mod confined;
mod effectexpr;
mod expr;
mod funcdef;
mod letexpr;
mod lookups;
mod matchexpr;
mod parens;
mod pattern;
mod proc;
mod pureexpr;
mod query;
mod restrict;

use derive_more::From;
use derive_new::new;
use sappho_ast_effect::{ProcEffect, PureEffect, QueryEffect};
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_object::Object;
use sappho_parsable::Recursive;
use sappho_primval::PrimVal;

use crate::leftassoc::LeftAssoc;
use crate::spanned::Spanned;

// Common internal parser recursion:
pub(crate) type ProcExprParser<'a> = Recursive<'a, ProcExpr>;

// Top-level expressions for each effect kind:
#[derive(Debug, From)]
pub struct PureExpr(Spanned<Expr<PureEffect>>);

#[derive(Debug, From)]
pub struct QueryExpr(Spanned<Expr<QueryEffect>>);

#[derive(Debug, From)]
pub struct ProcExpr(Spanned<Expr<ProcEffect>>);

// Potentially effectful expressions:
#[derive(Debug, From, new)]
pub struct EffectExpr<FX> {
    pub effects: Vec<FX>,
    pub confined: Confined<FX>,
}

// Generic structures across effects:
#[derive(Debug, From)]
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
    body: Box<PureExpr>,
}

#[derive(Clone, Debug, PartialEq, From)]
pub enum Pattern {
    Bind(BindPattern),
    LitEq(PrimVal),
    Unpack(Attrs<Pattern>),
    List(ListForm<Pattern, BindPattern>),
}

#[derive(Clone, Debug, PartialEq, From)]
pub struct BindPattern(RcId);

#[derive(Debug, From)]
pub struct QueryDef(Box<QueryExpr>);

#[derive(Debug, From)]
pub struct ProcDef(Box<ProcExpr>);

#[derive(Debug, new)]
pub struct Let<FX> {
    clauses: Vec<LetClause<FX>>,
    #[new(into)]
    inner: Box<Expr<FX>>,
}

#[derive(Debug, new)]
pub struct LetClause<FX> {
    binding: Pattern,
    #[new(into)]
    definition: Box<Expr<FX>>,
}

#[derive(Debug, new)]
pub struct Match<FX> {
    #[new(into)]
    candidate: Box<Expr<FX>>,
    clauses: Vec<MatchClause<FX>>,
}

#[derive(Debug, new)]
pub struct MatchClause<FX> {
    binding: Pattern,
    #[new(into)]
    consequent: Box<Expr<FX>>,
}

#[derive(Debug, From)]
pub struct Applications<FX>(LeftAssoc<Lookups<FX>, Application<FX>>);

#[derive(Debug, From)]
pub struct Application<FX>(Lookups<FX>);

#[derive(Debug, From)]
pub struct Lookups<FX>(LeftAssoc<EffectExpr<FX>, Lookup>);

#[derive(Debug, From)]
pub struct Lookup(RcId);

#[derive(Debug, From)]
pub enum Confined<FX> {
    Ref(RcId),
    Prim(PrimVal),
    Parens(ParensExpr<FX>),
    ObjectDef(Object<FuncDef, QueryDef, ProcDef, Expr<FX>>),
    ListExpr(ListForm<Expr<FX>, Box<Expr<FX>>>),
}

#[derive(Debug, From)]
pub struct ParensExpr<FX>(Box<Expr<FX>>);
