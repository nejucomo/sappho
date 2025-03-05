use crate::Syntax;

pub trait Parsable: From<<Self::Syntax as Syntax>::Parsed> {
    type Syntax: Syntax;

    fn syntax() -> Self::Syntax;
}
