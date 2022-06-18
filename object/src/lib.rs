use sappho_identmap::IdentMap;
use std::fmt;

#[derive(Debug)]
pub struct Object<F, Q, A> {
    pub func: Option<F>,
    pub query: Option<Q>,
    pub attrs: IdentMap<A>,
}

impl<F, Q, A> Object<F, Q, A> {
    pub fn transform<TF, F2, TQ, Q2, TA, A2>(
        self,
        tfunc: TF,
        tquery: TQ,
        tattr: TA,
    ) -> Object<F2, Q2, A2>
    where
        TF: Fn(F) -> F2,
        TQ: Fn(Q) -> Q2,
        TA: Fn(A) -> A2,
    {
        Object {
            func: self.func.map(tfunc),
            query: self.query.map(tquery),
            attrs: self.attrs.map_values(tattr),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.func.is_none() && self.query.is_none() && self.attrs.is_empty()
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
                    write!(f, ", ")
                } else {
                    self.0 = true;
                    Ok(())
                }
            }
        }

        let mut ct = CommaTracker(false);

        write!(f, "{{")?;
        if let Some(func) = self.func.as_ref() {
            ct.insert(f)?;
            func.fmt(f)?;
        }

        if let Some(query) = self.query.as_ref() {
            ct.insert(f)?;
            query.fmt(f)?;
        }

        for (name, attr) in self.attrs.iter() {
            ct.insert(f)?;
            write!(f, "{}: ", name)?;
            attr.fmt(f)?;
        }

        write!(f, " }}")
    }
}
