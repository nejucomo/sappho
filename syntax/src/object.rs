use sappho_parsable::ParsableWith;

use crate::procexpr::ProcExprParser;

impl<FX> ParsableWith<ProcExprParser<'_>> for Object<FX> {}
