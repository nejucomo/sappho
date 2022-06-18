use crate::{Result, ValRef};
use sappho_identmap::IdentMap;
use std::fmt;

pub struct Object {
    pub func: Option<Box<dyn Fn(ValRef) -> Result<ValRef>>>,
    pub query: Option<Box<dyn Fn() -> Result<ValRef>>>,
    pub attrs: IdentMap<ValRef>,
}

impl Object {
    fn is_empty(&self) -> bool {
        self.func.is_none() && self.query.is_none() && self.attrs.is_empty()
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "{{}}");
        }

        struct CommaTracker(bool);

        impl CommaTracker {
            pub fn insert(&mut self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
        if self.func.is_some() {
            ct.insert(f)?;
            write!(f, " fn … -> …")?;
        }

        if self.query.is_some() {
            ct.insert(f)?;
            write!(f, " query …")?;
        }

        for name in self.attrs.keys() {
            ct.insert(f)?;
            write!(f, " {}: …", name)?;
        }

        write!(f, " }}")
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
