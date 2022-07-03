use crate::UnpackPattern;
use sappho_ast as ast;
use test_case::test_case;

#[test_case(vec![] => UnpackPattern::from_iter(vec![]))]
fn transform_list_pattern(astpats: Vec<ast::Pattern>) -> UnpackPattern {
    UnpackPattern::from(ast::ListPattern::from_iter(astpats))
}
