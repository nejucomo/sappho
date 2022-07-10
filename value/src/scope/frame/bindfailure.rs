use crate::{CoercionFailure, ValRef};
use sappho_east::Pattern;
use std::fmt;

#[derive(Debug)]
pub struct BindFailure(Pattern, ValRef, BindFailureReason);

#[derive(Debug, derive_more::From)]
pub enum BindFailureReason {
    LitNotEqual,
    Coercion(CoercionFailure),
    MissingAttr(String),
    UnexpectedAttrs(Vec<String>),
}

impl BindFailure {
    pub fn new(pattern: &Pattern, val: &ValRef, reason: BindFailureReason) -> Self {
        BindFailure(pattern.clone(), val.clone(), reason)
    }
}

impl fmt::Display for BindFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let BindFailure(p, v, r) = self;
        write!(f, "value {} does not match pattern {}: {}", v, p, r)
    }
}

impl fmt::Display for BindFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BindFailureReason::*;

        match self {
            LitNotEqual => write!(f, "not equal"),
            Coercion(x) => x.fmt(f),
            MissingAttr(s) => write!(f, "missing attr {}", s),
            UnexpectedAttrs(v) => write!(f, "unexpected attrs {}", v.join(", ")),
        }
    }
}
