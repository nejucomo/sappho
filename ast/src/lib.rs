mod exprimpl;

use std::rc::Rc;

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
    pub body: Rc<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Application {
    pub target: Box<Expr>,
    pub argument: Box<Expr>,
}
