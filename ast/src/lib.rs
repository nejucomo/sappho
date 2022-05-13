use std::rc::Rc;

pub type Identifier = String;
pub type Pattern = Identifier;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Lit(Literal),
    Ref(Identifier),
    List(Vec<Expr>),
    Let(Box<LetExpr>),
    Func(FuncExpr),
    Apply(Box<Application>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Num(f64),
}

#[derive(Debug, PartialEq)]
pub struct LetExpr {
    pub binding: Pattern,
    pub bindexpr: Expr,
    pub tail: Expr,
}

#[derive(Debug, PartialEq)]
pub struct FuncExpr {
    pub binding: Pattern,
    pub body: Rc<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Application {
    pub target: Expr,
    pub argument: Expr,
}
