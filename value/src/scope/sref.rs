use std::rc::Rc;

use derive_more::Deref;
use sappho_ast_reduced::Pattern;

use crate::{BindFailure, Frame, Scope, ValRef};

#[derive(Clone, Debug, Deref)]
#[deref(forward)]
pub struct ScopeRef(Rc<Scope>);

impl Default for ScopeRef {
    fn default() -> Self {
        ScopeRef(Rc::new(Scope::Empty))
    }
}

impl ScopeRef {
    pub fn declare_then_bind(
        &self,
        pattern: &Pattern,
        value: &ValRef,
    ) -> Result<Self, BindFailure> {
        let subscope = self.declare([pattern]);
        subscope.bind_pattern(pattern, value)?;
        Ok(subscope)
    }

    pub fn declare<'a, I>(&self, patterns: I) -> Self
    where
        I: IntoIterator<Item = &'a Pattern>,
    {
        let mut frame = Frame::default();

        for pattern in patterns {
            frame.declare(pattern);
        }

        self.extend(frame)
    }

    fn extend(&self, frame: Frame) -> ScopeRef {
        let scope = Scope::Frame(frame, self.clone());
        ScopeRef(Rc::new(scope))
    }
}

impl PartialEq for ScopeRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
