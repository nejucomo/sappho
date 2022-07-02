use derive_new::new;
use sappho_identmap::IdentMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, new)]
pub struct Object<F, Q, A> {
    f: Option<F>,
    q: Option<Q>,
    a: IdentMap<A>,
}

#[derive(Debug, PartialEq)]
pub enum Monolithic<F, Q, A> {
    Func(F),
    Query(Q),
    Attrs(A),
}

impl<F, Q, A> Object<F, Q, A> {
    pub fn new_func(func: F) -> Self {
        Self::new(Some(func), None, IdentMap::default())
    }

    pub fn new_query(query: Q) -> Self {
        Self::new(None, Some(query), IdentMap::default())
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

    pub fn monolithic(&self) -> Option<Monolithic<&F, &Q, &IdentMap<A>>> {
        if let Some(f) = self.func() {
            if self.q.is_none() && self.a.is_empty() {
                Some(Monolithic::Func(f))
            } else {
                None
            }
        } else if let Some(q) = self.query() {
            if self.f.is_none() && self.a.is_empty() {
                Some(Monolithic::Query(q))
            } else {
                None
            }
        } else {
            Some(Monolithic::Attrs(self.attrs()))
        }
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
