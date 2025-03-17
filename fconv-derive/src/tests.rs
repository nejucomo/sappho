use indoc::indoc;
use proc_macro2::TokenStream;
use test_case::test_case;

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

#[test_case(
    TestCase {
        input: indoc! { r#"
            struct Foo(Bar);
        "# },
        expected: indoc! { r#"
            // FIXME
            struct Foo(Bar);
        "# },
    }
)]
fn extract_derive_pm2(TestCase { input, expected }: TestCase) {
    let input_tokens = parse_tokens(input);
    let expected_tokens = parse_tokens(expected);
    let actual_tokens = crate::extract_derive_pm2(input_tokens);
    let actual = actual_tokens.to_string();

    assert_eq!(
        // This is normalized compared to `expected`:
        expected_tokens.to_string(),
        actual,
        "Expected:\n{expected}\n\nActual:\n{actual}\n\n"
    );
}

fn parse_tokens(s: &str) -> TokenStream {
    s.parse().unwrap()
}
