use crate::Element;
use sappho_identmap::IdentMap;

pub struct IntoIter<F, Q, P, A> {
    pub(crate) f: Option<F>,
    pub(crate) q: Option<Q>,
    pub(crate) p: Option<P>,
    pub(crate) a: <IdentMap<A> as IntoIterator>::IntoIter,
}

impl<F, Q, P, A> Iterator for IntoIter<F, Q, P, A> {
    type Item = Element<F, Q, P, A>;

    fn next(&mut self) -> Option<Self::Item> {
        use Element::*;

        if let Some(f) = self.f.take() {
            Some(Func(f))
        } else if let Some(q) = self.q.take() {
            Some(Query(q))
        } else if let Some(p) = self.p.take() {
            Some(Proc(p))
        } else {
            self.a.next().map(|(k, v)| Attr(k, v))
        }
    }
}
