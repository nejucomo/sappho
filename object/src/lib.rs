use sappho_identmap::IdentMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Object<F, Q, A> {
    f: Option<F>,
    q: Option<Q>,
    a: IdentMap<A>,
}

impl<F, Q, A> Object<F, Q, A> {
    pub fn new(func: Option<F>, query: Option<Q>, attrs: IdentMap<A>) -> Self {
        Object {
            f: func,
            q: query,
            a: attrs,
        }
    }

    pub fn func(&self) -> Option<&F> {
        self.f.as_ref()
    }

    pub fn query(&self) -> Option<&Q> {
        self.q.as_ref()
    }

    pub fn attrs(&self) -> &IdentMap<A> {
        &self.a
    }

    pub fn is_empty(&self) -> bool {
        self.f.is_none() && self.q.is_none() && self.a.is_empty()
    }

    pub fn transform<TF, FR, TQ, QR, TA, AR>(
        self,
        tfunc: TF,
        tquery: TQ,
        tattr: TA,
    ) -> Object<FR, QR, AR>
    where
        TF: FnOnce(F) -> FR,
        TQ: FnOnce(Q) -> QR,
        TA: Fn(A) -> AR,
    {
        Object {
            f: self.f.map(tfunc),
            q: self.q.map(tquery),
            a: self.a.into_map_values(tattr),
        }
    }
}

impl<F, Q, A> fmt::Display for Object<F, Q, A>
where
    F: fmt::Display,
    Q: fmt::Display,
    A: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "{{}}");
        }

        struct CommaTracker(bool);

        impl CommaTracker {
            pub fn insert(&mut self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.0 {
                    write!(f, ",")
                } else {
                    self.0 = true;
                    Ok(())
                }
            }
        }

        let mut ct = CommaTracker(false);

        write!(f, "{{")?;
        if let Some(func) = self.func() {
            ct.insert(f)?;
            write!(f, " ")?;
            func.fmt(f)?;
        }

        if let Some(query) = self.query() {
            ct.insert(f)?;
            write!(f, " ")?;
            query.fmt(f)?;
        }

        for (name, attr) in self.attrs().iter() {
            ct.insert(f)?;
            write!(f, " {}: ", name)?;
            attr.fmt(f)?;
        }

        write!(f, " }}")
    }
}
