use crate::Object;
use sappho_identmap::IdentMap;

#[derive(Debug, PartialEq)]
pub enum Unbundled<F, Q, A> {
    Bundled(Object<F, Q, A>),
    Func(F),
    Query(Q),
    Attrs(IdentMap<A>),
}
