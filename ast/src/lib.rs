//! This Abstract Syntax Tree corresponds to the textual grammar of `saplang`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation. Example:
//!
//! `fn x -> x` is short-hand for `{ fn x -> x }`.
mod exprimpl;

pub type Identifier = String;
pub type Pattern = Identifier;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Lit(Literal),
    Ref(Identifier),
    List(Vec<Expr>),
    Let(LetExpr),
    Func(FuncExpr),
    Apply(Application),
    Object(ObjectExpr),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Num(f64),
}

#[derive(Debug, PartialEq)]
pub struct LetExpr {
    pub binding: Pattern,
    pub bindexpr: Box<Expr>,
    pub tail: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct FuncExpr {
    pub binding: Pattern,
    pub body: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Application {
    pub target: Box<Expr>,
    pub argument: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectExpr {
    pub func: Option<FuncExpr>,
}
