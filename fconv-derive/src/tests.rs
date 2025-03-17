use indoc::indoc;
use proc_macro2::TokenStream;
use test_case::{test_case, test_matrix};

struct TestCase<S> {
    input: S,
    expected: S,
}

impl<S> TestCase<S>
where
    S: AsRef<str>,
{
    fn as_strs(&self) -> TestCase<&str> {
        TestCase {
            input: self.input.as_ref(),
            expected: self.expected.as_ref(),
        }
    }
}

impl TestCase<&str> {
    fn check_expansion(self) {
        let TestCase { input, expected } = self;
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
}

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
fn extract_derive_pm2(tc: TestCase<&str>) {
    tc.check_expansion()
}

// We want to test a cartesian produce:
// - struct-vs-enum
// - indexed-vs-named fields
// - single-field-vs-multi-field (? via tuples like `derive_more::From` ?)
// - concrete-vs-generic-with-bounds

// Test Matrix Axes Types:
#[derive(Debug, Copy, Clone)]
enum Kind {
    Struct,
    Enum,
}
use Kind::*;

#[derive(Debug, Copy, Clone)]
enum FieldSpec {
    Indexed,
    Named,
}
use FieldSpec::*;

#[derive(Debug, Copy, Clone)]
enum Genericity {
    Concrete,
    Generic,
}
use Genericity::*;

// Maybe TODO?
// enum FieldCount { Single, Multiple }

#[test_matrix(
    [Struct, Enum],
    [Indexed, Named],
    [Concrete, Generic]
)]
fn matrix(kind: Kind, fspec: FieldSpec, gen: Genericity) {
    TestCase {
        input: matrix_input(kind, fspec, gen),
        expected: matrix_expected(kind, fspec, gen),
    }
    .as_strs()
    .check_expansion()
}

fn matrix_input(kind: Kind, fspec: FieldSpec, gen: Genericity) -> String {
    let mut input = "".to_string();
    input.push_str(match kind {
        Struct => "struct",
        Enum => "enum",
    });
    input.push_str(" Foo");
    if matches!(gen, Generic) {
        input.push_str("<T>");
    }
    input.push_str(match (kind, fspec) {
        (Struct, Indexed) => "(T);",
        (Struct, Named) => indoc! { r#"
             {
              t: T
            }
        "# },
        (Enum, Indexed) => indoc! { r#"
             {
              Thing(T),
              NotThing(bool),
            }
        "# },
        (Enum, Named) => indoc! { r#"
             {
              Thing { t: T },
              NotThing(bool),
            }
        "# },
    });
    input
}

fn matrix_expected(kind: Kind, fspec: FieldSpec, gen: Genericity) -> String {
    match (kind, fspec, gen) {
        (Struct, Indexed, Concrete) => indoc! { r#"
            #[automatically_derived]
            impl ::sappho_fconv::Extract<T> for Foo {
                fn extract(self) -> Result<T, Self> {
                    Ok(self.0)
                }
            }

            #[automatically_derived]
            impl ::sappho_fconv::Embed<T> for Foo {
                fn embed(thing: T) -> Self {
                    Foo(thing)
                }
            }
        "# }
        .to_string(),
        (Struct, Indexed, Generic) => todo!(),
        (Struct, Named, Concrete) => todo!(),
        (Struct, Named, Generic) => todo!(),
        (Enum, Indexed, Concrete) => todo!(),
        (Enum, Indexed, Generic) => todo!(),
        (Enum, Named, Concrete) => todo!(),
        (Enum, Named, Generic) => todo!(),
    }
}

fn parse_tokens(s: &str) -> TokenStream {
    s.parse().unwrap()
}
