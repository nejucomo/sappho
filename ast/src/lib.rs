use derive_more::From;

#[derive(Debug, From, PartialEq)]
pub enum Expr {
    Lit(Literal),
}

#[derive(Debug, From, PartialEq)]
pub enum Literal {
    Num(f64),
}
