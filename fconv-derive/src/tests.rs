use indoc::indoc;
use proc_macro2::TokenStream;
use test_case::test_case;

struct TestCase {
    input: &'static str,
    expected: &'static str,
}

// We want to test a cartesian produce:
// - struct-vs-enum
// - indexed-vs-named fields
// - single-field-vs-multi-field (? via tuples like `derive_more::From` ?)
// - concrete-vs-generic-with-bounds
#[test_case(
    TestCase {
        input: indoc! { r#"
            struct Foo(Bar);
        "# },
        expected: indoc! { r#"
            #[automatically_derived]
            impl ::sappho_fconv::Extract<Bar> for Foo {
                fn extract(self) -> Result<Bar, Self> {
                    Ok(self.0)
                }
            }

            #[automatically_derived]
            impl ::sappho_fconv::Embed<Bar> for Foo {
                fn embed(thing: Bar) -> Self {
                    Foo(thing)
                }
            }
        "# },
    }
    ; "concrete-struct-1-indexed-field"
)]
#[test_case(
    TestCase {
        input: indoc! { r#"
            enum Foo {
                MkBar(Bar),
                MkBool(bool),
            }
        "# },
        expected: indoc! { r#"
            #[automatically_derived]
            impl ::sappho_fconv::Extract<Bar> for Foo {
                fn extract(self) -> Result<Bar, Self> {
                    match self {
                        Foo::MkBar(x) => Ok(x),
                        other => Err(other),
                    }
                }
            }

            #[automatically_derived]
            impl ::sappho_fconv::Embed<Bar> for Foo {
                fn embed(thing: Bar) -> Self {
                    Foo::MkBar(thing)
                }
            }

            #[automatically_derived]
            impl ::sappho_fconv::Extract<bool> for Foo {
                fn extract(self) -> Result<bool, Self> {
                    match self {
                        Foo::MkBool(x) => Ok(x),
                        other => Err(other),
                    }
                }
            }

            #[automatically_derived]
            impl ::sappho_fconv::Embed<bool> for Foo {
                fn embed(thing: bool) -> Self {
                    Foo::MkBool(thing)
                }
            }
        "# },
    }
    ; "concrete-enum-1-indexed-fields"
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
