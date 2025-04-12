use std::fmt;
use std::rc::Rc;

use derive_more::{Deref, DerefMut, From};
use sappho_attrs::errors::Missing;
use sappho_attrs::Attrs;
use sappho_east::{FuncDef, ProcDef, QueryDef};
use sappho_identifier::RcId;
use sappho_object::Object;

use crate::{AsError, VResult, Valuable, Value};

#[derive(Clone, Debug, PartialEq, From, Deref)]
#[from(ObjectVal)]
#[deref(forward)]
pub struct ObjectRc(Rc<ObjectVal>);

#[derive(Clone, Debug, PartialEq, Deref, DerefMut, From)]
pub struct ObjectVal(Object<Rc<FuncDef>, Rc<QueryDef>, Rc<ProcDef>, Value>);

impl ObjectVal {
    pub fn new<F, Q, P, A>(f: F, q: Q, p: P, a: A) -> Self
    where
        F: Into<Option<FuncDef>>,
        Q: Into<Option<QueryDef>>,
        P: Into<Option<ProcDef>>,
        A: Into<Attrs<Value>>,
    {
        ObjectVal(Object::new(
            f.into().map(Rc::new),
            q.into().map(Rc::new),
            p.into().map(Rc::new),
            a,
        ))
    }
}

impl Valuable for ObjectVal {
    fn attr_lookup<'s>(&'s self, name: &RcId) -> VResult<&'s Value, Missing> {
        self.attrs().get(name).map_err(|e| self.wrap_error(e))
    }

    fn apply(&self, argument: Value) -> VResult<Value, AsError> {
        if let Some(funcdef) = self.func() {
            // BUG: We need cyclic dependency on `eval`
            let _ = (argument, funcdef);
            todo!("figure out how to manage eval<->value<->ast interdependency.")
        } else {
            Err(self.wrap_error(AsError("fn")))
        }
    }
}

impl fmt::Display for ObjectVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Fix this to a less lazy, more "native" impl:
        write!(f, "{self:#?}")
    }
}
