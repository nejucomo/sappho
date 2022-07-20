use crate::Object;
use sappho_identmap::IdentMap;

/// An object containing exactly one of func, query, proc, or attrs can be "unbundled" into that
/// component. This is used for canonicalization, e.g. `{ fn x -> x }` -> `fn x -> x`.
#[derive(Debug, PartialEq)]
pub enum Unbundled<F, Q, P, A> {
    Bundled(Object<F, Q, P, A>),
    Func(F),
    Query(Q),
    Proc(P),
    Attrs(IdentMap<A>),
}
