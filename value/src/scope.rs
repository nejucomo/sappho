use derive_more::Deref;
use sappho_attrs::Attrs;
use sappho_list::List;

use crate::Value;

/// # TODO
///
/// Replace this with pre-runtime { RcId -> index } syntactic info and `Vec<Value>`
#[derive(Clone, Debug, Default, PartialEq, Deref)]
pub struct Scope(List<Locals>);

pub type Locals = Attrs<Value>;
