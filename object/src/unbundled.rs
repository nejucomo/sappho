use std::fmt::Debug;

use sappho_attrs::Attrs;

use crate::Object;

/// An object containing exactly one of func, query, proc, or attrs can be "unbundled" into that
/// component. This is used for canonicalization, e.g. `{ fn x -> x }` -> `fn x -> x`.
#[derive(Debug, PartialEq)]
pub enum Unbundled<F, Q, P, A>
where
    A: Debug,
{
    Bundled(Object<F, Q, P, A>),
    Func(F),
    Query(Q),
    Proc(P),
    Attrs(Attrs<A>),
}

impl<F, Q, P, A> From<Unbundled<F, Q, P, A>> for Object<F, Q, P, A>
where
    A: Debug,
{
    fn from(ub: Unbundled<F, Q, P, A>) -> Self {
        use Unbundled::*;

        match ub {
            Bundled(obj) => obj,
            Func(f) => Object::new_func(f),
            Query(q) => Object::new_query(q),
            Proc(p) => Object::new_proc(p),
            Attrs(attrs) => Object::new_attrs(attrs),
        }
    }
}
