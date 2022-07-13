use crate::Stream;

pub trait Unparse {
    fn unparse(&self) -> Stream;
}
