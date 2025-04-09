use std::fmt::{Debug, Display};

use sappho_attrs::errors::Missing;
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_primval::PrimVal;

use crate::{CastTo, FuncVal, ObjectVal, PseudoType, PseudoTypeError, VResult, Value, ValueError};

/// All of the user-space operations possible with a value
pub trait Valuable:
    Clone
    + Debug
    + Display
    + PartialEq
    + Into<Value>
    + CastTo<PrimVal>
    + CastTo<ObjectVal>
    + CastTo<FuncVal>
    + CastTo<List<Value>>
{
    fn attr_lookup<'s>(&'s self, name: &RcId) -> VResult<&'s Value, Missing> {
        Err(self.wrap_error(Missing::from(name)))
    }

    fn cast<T>(&self) -> VResult<&T, PseudoTypeError>
    where
        Self: CastTo<T>,
        T: PseudoType,
    {
        self.wrap_res(CastTo::<T>::cast(self))
    }

    /// # TODO
    ///
    /// Make a more principled/universal type system.
    fn clone_into_value(&self) -> Value {
        self.clone().into()
    }

    fn wrap_error<E>(&self, inner: E) -> ValueError<E> {
        ValueError::new(self.clone_into_value(), inner)
    }

    fn wrap_res<T, E>(&self, res: Result<T, E>) -> VResult<T, E> {
        res.map_err(|e| self.wrap_error(e))
    }
}
