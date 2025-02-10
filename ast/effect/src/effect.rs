use crate::{ProcEffect, PureEffect, QueryEffect};

use self::sealed::Sealed;

pub trait Effect: Sealed {}

mod sealed {
    pub trait Sealed {}
}

impl Sealed for PureEffect {}

impl Sealed for QueryEffect {}

impl Sealed for ProcEffect {}
