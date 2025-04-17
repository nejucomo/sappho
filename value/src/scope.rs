use sappho_identifier::RcId;
use thiserror::Error;

use crate::{Locals, Scoped, Value};

/// A lexical scope.
///
/// # TODO
///
/// Replace this with pre-runtime { RcId -> index } syntactic info and `Vec<Value>`
#[derive(Debug, Default)]
pub struct Scope {
    locals: Locals,
    outers: Option<Box<Scope>>,
}

#[derive(Clone, Debug, Error)]
#[error("lookup error for `{reference}`: {reason}")]
pub struct LookupError {
    reference: RcId,
    reason: LookupErrorReason,
}

#[derive(Clone, Debug, Error)]
pub enum LookupErrorReason {
    #[error("not in scope")]
    NotInScope,
    #[error("undefined")]
    Undefined,
}

impl Scope {
    pub fn lookup(&self, id: &RcId) -> Result<&Value, LookupError> {
        self.lookup_inner(id).map_err(|reason| LookupError {
            reference: id.clone(),
            reason,
        })
    }

    fn lookup_inner(&self, id: &RcId) -> Result<&Value, LookupErrorReason> {
        use LookupErrorReason::*;

        for locals in self.iter_locals() {
            if let Some(optvref) = locals.lookup_opt(id) {
                return optvref.ok_or(Undefined);
            }
        }

        Err(NotInScope)
    }

    fn iter_locals(&self) -> RefIter {
        RefIter(Some(self))
    }

    pub fn wrap<T>(self, other: T) -> Scoped<T> {
        Scoped::new(self, other)
    }
}

struct RefIter<'s>(Option<&'s Scope>);

impl<'s> Iterator for RefIter<'s> {
    type Item = &'s Locals;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|s| {
            let item = &s.locals;
            self.0 = s.outers.as_ref().map(|boxscope| boxscope.as_ref());
            item
        })
    }
}
