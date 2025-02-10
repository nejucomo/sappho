use sappho_unparse::Unparse;

use self::sealed::Sealed;

pub trait Effect: Sealed + Unparse + PartialEq {}

mod sealed {
    use crate::{ProcEffect, PureEffect, QueryEffect};

    pub trait Sealed {}

    impl Sealed for PureEffect {}

    impl Sealed for QueryEffect {}

    impl Sealed for ProcEffect {}
}
