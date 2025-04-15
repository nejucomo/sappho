use std::rc::Rc;

use sappho_effect::Effect;
use sappho_source::SourceCodeRef;
use sappho_syntax as syntax;

use crate::transform::{TransformInto as _, TransformWithSource};
use crate::Wise;

impl<FX> TransformWithSource<Wise<FX>> for syntax::Expr<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: &Option<SourceCodeRef>) -> Wise<FX> {
        use syntax::Expr::*;

        match self {
            Func(x) => Wise::new(Rc::new(x.transform_into()), src.clone()),
            Query(x) => Wise::new(Rc::new(x.transform_into()), src.clone()),
            Proc(x) => Wise::new(Rc::new(x.transform_into()), src.clone()),
            Let(x) => Wise::new(x.transform_into(), src.clone()),
            Match(x) => Wise::new(x.transform_into(), src.clone()),
            Applications(x) => x.transform_with_source(src),
        }
    }
}
