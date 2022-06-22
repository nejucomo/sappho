use crate::ValRef;
use sappho_east::Pattern;
use std::fmt;

#[derive(Debug)]
pub struct BindFailure(Pattern, ValRef);

impl BindFailure {
    pub fn new(pattern: &Pattern, val: &ValRef) -> Self {
        BindFailure(pattern.clone(), val.clone())
    }
}

impl fmt::Display for BindFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let BindFailure(p, v) = self;
        write!(f, "value {} does not match pattern {}", v, p)
    }
}
