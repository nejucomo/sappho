use crate::{BindFailure, Frame, Scope, ValRef};
use sappho_ast_reduced::Pattern;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
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

impl Deref for ScopeRef {
    type Target = Scope;

    fn deref(&self) -> &Scope {
        self.0.deref()
    }
}
