use crate::{Result, Stream};

pub trait UnparseWrappable {
    fn unparse_wrappable<S>(&self, stream: &mut S, wrap: bool) -> Result<()>
    where
        S: Stream;

    fn should_wrap_if_unfit(&self) -> bool;
}
