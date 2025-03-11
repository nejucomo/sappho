use std::fmt::Debug;

use sappho_unparse::Unparse;

use crate::EffectDescription;

use self::sealed::Sealed;

pub trait Effect: Sealed + Unparse + PartialEq + Copy + Debug {
    /// The context these effects are restricted to occur in
    fn context() -> &'static str;

    /// The user-facing descriptors for this effect
    fn description(self) -> EffectDescription;

    /// The symbol prefix used to express this effect in an expression
    fn sigil(self) -> &'static str {
        self.description().sigil
    }

    /// The noun name of this effect
    fn noun(self) -> &'static str {
        self.description().noun
    }

    /// The infinitive verb which fulfills "the expression `<symbol>x` will <infinitive> the value, `x`"
    fn verb(self) -> &'static str {
        self.description().verb
    }
}

mod sealed {
    use crate::{ProcEffect, PureEffect, QueryEffect};

    pub trait Sealed {}

    impl Sealed for PureEffect {}

    impl Sealed for QueryEffect {}

    impl Sealed for ProcEffect {}
}
