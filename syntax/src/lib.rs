#![allow(dead_code)]

pub mod leftassoc;

use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_object::Object;
use sappho_primval::PrimVal;

use crate::leftassoc::LeftAssoc;

pub type PureExpr = Expr<PureConfined>;

#[derive(Debug)]
pub struct PureConfined(Confined<Self>);

pub type QueryExpr = Expr<QueryConfined>;

#[derive(Debug)]
pub enum QueryConfined {
    Inquiry(Box<Self>),
    NoFx(Confined<Self>),
}

pub type ProcExpr = Expr<ProcConfined>;

#[derive(Debug)]
pub enum ProcConfined {
    Invocation(Box<Self>),
    Inquiry(Box<Self>),
    NoFx(Confined<Self>),
}

#[derive(Debug)]
pub enum Expr<FX> {
    Func(FuncDef),
    Query(QueryDef),
    Proc(ProcDef),
    Let(Let<FX>),
    Match(Match<FX>),
    Applications(Applications<FX>),
}

#[derive(Debug)]
pub struct FuncDef {
    argpat: Pattern,
    body: Box<PureExpr>,
}

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(RcId),
    LitEq(PrimVal),
    Unpack(Attrs<Pattern>),
    List(ListForm<Pattern, RcId>),
}

#[derive(Debug)]
pub struct QueryDef(Box<QueryExpr>);

#[derive(Debug)]
pub struct ProcDef(Box<ProcExpr>);

#[derive(Debug)]
pub struct Let<FX> {
    clauses: Vec<LetClause<FX>>,
    inner: Box<Expr<FX>>,
}

#[derive(Debug)]
pub struct LetClause<FX> {
    binding: Pattern,
    definition: Box<Expr<FX>>,
}

#[derive(Debug)]
pub struct Match<FX> {
    candidate: Box<Expr<FX>>,
    clauses: Vec<MatchClause<FX>>,
}

#[derive(Debug)]
pub struct MatchClause<FX> {
    binding: Pattern,
    consequent: Box<Expr<FX>>,
}

#[derive(Debug)]
pub struct Applications<FX>(LeftAssoc<Lookups<FX>, Lookups<FX>>);

#[derive(Debug, derive_more::From)]
pub struct Lookups<FX>(LeftAssoc<FX, Lookup>);

#[derive(Debug, derive_more::From)]
pub struct Lookup(RcId);

#[derive(Debug, derive_more::From)]
pub enum Confined<FX> {
    Ref(RcId),
    Prim(PrimVal),
    Parens(ParensExpr<FX>),
    ObjectDef(Object<FuncDef, QueryDef, ProcDef, Expr<FX>>),
    ListExpr(ListForm<Expr<FX>, Box<Expr<FX>>>),
}

#[derive(Debug, derive_more::From)]
pub struct ParensExpr<FX>(Box<Expr<FX>>);
