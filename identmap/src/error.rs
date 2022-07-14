use crate::Identifier;

#[derive(Debug)]
pub struct RedefinitionError(pub Identifier);
