use derive_more::From;
use sappho_ast_effect::Effect;
use sappho_unparse::Unparse;

use crate::{AstProvider, Wise};

/// Heap-allocated [Wise]
#[derive(Clone, Debug, PartialEq, From)]
pub struct BoxWise<XP, FX>(Box<Wise<XP, FX>>)
where
    XP: AstProvider,
    FX: Effect;

impl<XP, FX> Unparse for BoxWise<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.0.unparse_into(s)
    }
}
