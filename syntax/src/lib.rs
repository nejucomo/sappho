#![allow(dead_code)]

pub mod leftassoc;

use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_object::Object;
use sappho_primval::PrimVal;

use crate::leftassoc::LeftAssoc;

#[derive(Debug)]
pub enum Expr {
    Func(FuncDef),
    Query(QueryDef),
    Proc(ProcDef),
    Let(Let),
    Match(Match),
    Applications(Applications),
}

#[derive(Debug)]
pub struct FuncDef {
    argpat: Pattern,
    body: Box<Expr>,
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

pub type QueryExpr = Expr; // FIXME

#[derive(Debug)]
pub struct ProcDef(Box<ProcExpr>);

pub type ProcExpr = Expr; // FIXME

#[derive(Debug)]
pub struct Let {
    clauses: Vec<LetClause>,
    inner: Box<Expr>,
}

#[derive(Debug)]
pub struct LetClause {
    binding: Pattern,
    definition: Box<Expr>,
}

#[derive(Debug)]
pub struct Match {
    candidate: Box<Expr>,
    clauses: Vec<MatchClause>,
}

#[derive(Debug)]
pub struct MatchClause {
    binding: Pattern,
    consequent: Box<Expr>,
}

#[derive(Debug)]
pub struct Applications(LeftAssoc<Lookups, Lookups>);

#[derive(Debug, derive_more::From)]
pub struct Lookups(LeftAssoc<Confined, Lookup>);

#[derive(Debug, derive_more::From)]
pub struct Lookup(RcId);

#[derive(Debug, derive_more::From)]
pub enum Confined {
    Ref(RcId),
    Prim(PrimVal),
    Parens(ParensExpr),
    ObjectDef(Object<FuncDef, QueryDef, ProcDef, Expr>),
    ListExpr(ListForm<Expr, Box<Expr>>),
}

#[derive(Debug)]
pub struct QConfined {
    is_inquiry: bool,
    inner: Confined,
}

#[derive(Debug, derive_more::From)]
pub struct PConfined {
    is_invocation: bool,
    inner: QConfined,
}

#[derive(Debug, derive_more::From)]
pub struct ParensExpr(Box<Expr>);
