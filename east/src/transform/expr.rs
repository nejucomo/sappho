use sappho_effect::Effect;
use sappho_source::SourceCodeRef;
use sappho_syntax as syntax;
use sappho_with_source::WithSource;

use crate::transform::{TransformInto, TransformWithSource};
use crate::Expr;

impl<FX> TransformWithSource<Expr<FX>> for syntax::Expr<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: Option<SourceCodeRef>) -> WithSource<Expr<FX>> {
        use syntax::Expr::*;

        match self {
            Func(x) => WithSource::new(Expr::from(x.transform_into()), src),
            Query(x) => WithSource::new(Expr::from(x.transform_into()), src),
            Proc(x) => WithSource::new(Expr::from(x.transform_into()), src),
            Let(x) => WithSource::new(Expr::from(x.transform_into()), src),
            Match(x) => WithSource::new(Expr::from(x.transform_into()), src),
            Applications(x) => x.transform_with_source(src),
        }
    }
}
