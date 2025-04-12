use derive_new::new;
use easy_ext::ext;
use sappho_value::Locals;

use crate::step::{ContinueStep, Step};

#[derive(Debug, new)]
pub struct WithLocals<T> {
    pub(crate) expr: T,
    pub(crate) optlocals: Option<Locals>,
}

#[ext(LocalsWithT)]
impl Locals {
    fn with<T>(self, thing: T) -> WithLocals<T> {
        WithLocals::new(thing, Some(self))
    }
}

impl<T> WithLocals<T> {
    pub(crate) fn continue_to<V, C>(self, cont: C) -> Step<V, Self, C> {
        ContinueStep::new(self, cont).into()
    }
}
