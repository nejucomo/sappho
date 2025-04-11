use derive_more::From;

use crate::Eval;

#[derive(Debug, From)]
pub struct Scope<'s, X>(Vec<X::Locals>)
where
    X: Eval<'s>;

impl<'s, X> Default for Scope<'s, X>
where
    X: Eval<'s>,
{
    fn default() -> Self {
        Scope(vec![])
    }
}

impl<'s, X> Scope<'s, X>
where
    X: Eval<'s>,
{
    fn iter(&self) -> impl Iterator<Item = &X::Locals> {
        self.0.iter().rev()
    }

    pub(crate) fn push_locals(&mut self, locals: X::Locals) {
        self.0.push(locals);
    }

    pub(crate) fn pop_locals(&mut self) {
        self.0.pop().expect("lexical scope stack under-run");
    }
}
