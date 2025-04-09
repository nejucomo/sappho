use std::fmt::{Debug, Display};

use sappho_attrs::errors::Missing;
use sappho_east::PureExpr;
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_primval::PrimVal;

use crate::error::ValueResultExt as _;
use crate::funcval::ApplicationFailure;
use crate::{
    CastTo, FuncVal, ObjectVal, PseudoType, PseudoTypeError, Scope, VResult, Value, ValueError,
};

/// All of the user-space operations possible with a value
///
/// # TODO
///
/// Reconsider this trait, pseudo-types, duck-typing etc...
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

    fn apply<F, T>(&self, eval: F, arg: Value) -> VResult<T, ApplicationFailure>
    where
        F: FnOnce(Scope, &PureExpr) -> T,
    {
        let f = self.cast::<FuncVal>().convert_inner_err()?;
        let t = f.apply(eval, arg).convert_inner_err()?;
        Ok(t)
    }

    fn cast<T>(&self) -> VResult<&T, PseudoTypeError>
    where
        Self: CastTo<T>,
        T: PseudoType,
    {
        self.wrap_fres(|| self.cast_opt().ok_or(T::pseudo_type_error()))
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

    fn wrap_fres<F, T, E>(&self, f: F) -> VResult<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        f().map_err(|e| self.wrap_error(e))
    }
}
