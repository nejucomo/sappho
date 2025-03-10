use crate::{Kernel, Root};

/// A [QueryX] is an expression with query effects
#[derive(Debug, derive_more::From)]
pub enum QueryX {
    Kernel(Kernel<QueryX>),
    Inquire(Box<QueryX>),
}

impl Root for QueryX {}
