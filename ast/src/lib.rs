//! This Abstract Syntax Tree corresponds to the textual grammar of `saplang`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation. Example:
//!
//! `fn x -> x` is short-hand for `{ fn x -> x }`.
mod exprimpl;

pub type Identifier = String;
pub type Pattern = Identifier;

pub type Expr = GenExpr<PureEffects>;

#[derive(Debug, PartialEq)]
pub enum GenExpr<Effects> {
    Lit(Literal),
    Ref(Identifier),
    List(Vec<GenExpr<Effects>>),
    Let(LetExpr<Effects>),
    Func(FuncExpr),
    Apply(Application<Effects>),
    Query(QueryExpr),
    Object(ObjectExpr),
    Effect(Effects),
}

// Effects:
/// There are no `PureEffects` beyond the base `GenExpr`.
#[derive(Debug, PartialEq)]
pub enum PureEffects {}

#[derive(Debug, PartialEq)]
pub enum QueryEffects {
    Inquire(Box<GenExpr<QueryEffects>>),
}

#[derive(Debug, PartialEq)]
pub enum ProcEffects {
    Inquire(Box<GenExpr<ProcEffects>>),
    Evoke(Box<GenExpr<ProcEffects>>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Num(f64),
}

#[derive(Debug, PartialEq)]
pub struct LetExpr<Effects> {
    pub binding: Pattern,
    pub bindexpr: Box<GenExpr<Effects>>,
    pub tail: Box<GenExpr<Effects>>,
}

#[derive(Debug, PartialEq)]
pub struct FuncExpr {
    pub binding: Pattern,
    pub body: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Application<Effects> {
    pub target: Box<GenExpr<Effects>>,
    pub argument: Box<GenExpr<Effects>>,
}

#[derive(Debug, PartialEq)]
pub struct QueryExpr {
    pub body: Box<GenExpr<QueryEffects>>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectExpr {
    pub query: Option<QueryExpr>,
    pub func: Option<FuncExpr>,
}
