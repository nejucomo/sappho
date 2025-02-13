use sappho_ast_comments::Commented;
use sappho_unparse::Unparse;

/// A trait approximating [From] for transforming AST types
///
/// We would rather use `MyCoreAstThingy<XPD, FX>: From<MyCoreAstThingy<XPS, FX>>`, but the blanket impl of `From<T>` prevents this.
pub trait AstTransformInto<T> {
    fn ast_transform(self) -> T;
}

/// When a type already provides a suitable [Into] impl, use this to impl [AstTransformInto] via [Into] delegation
#[macro_export]
macro_rules! ast_transform_into_via_into {
    ( $t:ty ) => {
        impl<T> AstProvider<T> for $t
        where
            Self: Into<T>,
        {
            fn ast_transform(self) -> T {
                self.into()
            }
        }
    };
}

impl<S, T> AstTransformInto<Box<T>> for Box<S>
where
    S: AstTransformInto<T>,
{
    fn ast_transform(self) -> Box<T> {
        Box::new((*self).ast_transform())
    }
}

impl<S, T> AstTransformInto<Commented<T>> for Commented<S>
where
    Self: Unparse,
    S: AstTransformInto<T>,
{
    fn ast_transform(self) -> Commented<T> {
        let unparse = self.unparse();
        self.map(S::ast_transform)
            .with_appended_comment_section("AST Transform Source", unparse)
    }
}
