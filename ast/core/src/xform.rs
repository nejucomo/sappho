use sappho_ast_comments::Commented;
use sappho_identmap::IdentMap;
use sappho_unparse::Unparse;

/// A trait approximating [From] for transforming AST types
///
/// We would rather use `MyCoreAstThingy<XPD, FX>: From<MyCoreAstThingy<XPS, FX>>`, but the blanket impl of `From<T>` prevents this.
pub trait AstTransformInto<T> {
    fn ast_transform(self) -> T;
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

impl<S, T> AstTransformInto<IdentMap<T>> for IdentMap<S>
where
    S: AstTransformInto<T>,
{
    fn ast_transform(self) -> IdentMap<T> {
        self.into_map_values(S::ast_transform)
    }
}
