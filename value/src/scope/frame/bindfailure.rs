use crate::{CoercionFailure, ValRef};
use sappho_ast_reduced::Pattern;
use sappho_identifier::RcId;
use std::fmt;

#[derive(Debug)]
pub struct BindFailure(Pattern, ValRef, BindFailureReason);

#[derive(Debug, derive_more::From)]
pub enum BindFailureReason {
    LitNotEqual,
    Coercion(CoercionFailure),
    MissingAttr(RcId),
    UnexpectedAttrs(Vec<RcId>),
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
            UnexpectedAttrs(v) => write!(
                f,
                "unexpected attrs {}",
                // Hrm... `RcId` is cumbersome here unless we could `impl std::slice::Join<&str> for [RcId]` ?
                v.into_iter()
                    .map(|rcid| rcid.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
