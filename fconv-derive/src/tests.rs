use indoc::indoc;
use proc_macro2::TokenStream;
use syn::Ident;
use test_case::{test_case, test_matrix};

use crate::FieldSpec::{self, *};

use self::Genericity::*;
use self::Kind::*;

#[derive(Debug, Copy, Clone)]
enum Genericity {
    Concrete,
    Generic,
}

#[derive(Debug, Copy, Clone)]
enum Kind {
    Struct,
    Enum,
}

#[derive(Debug)]
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
            impl ::sappho_fconv::Extract<Bar> for (Foo) {
                fn extract(self) -> Result<Bar, Self> {
                    match self {
                        Foo(x) => Ok(x),
                    }
                }
            }

            #[automatically_derived]
            impl ::sappho_fconv::Embed<Bar> for (Foo) {
                fn embed(x: Bar) -> Self {
                    Foo(x)
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
            impl ::sappho_fconv::Extract<Bar> for (Foo) {
                fn extract(self) -> Result<Bar, Self> {
                    match self {
                        Foo::MkBar(x) => Ok(x),
                        other => Err(other)
                    }
                }
            }

            #[automatically_derived]
            impl ::sappho_fconv::Embed<Bar> for (Foo) {
                fn embed(x: Bar) -> Self {
                    Foo::MkBar(x)
                }
            }

            #[automatically_derived]
            impl ::sappho_fconv::Extract<bool> for (Foo) {
                fn extract(self) -> Result<bool, Self> {
                    match self {
                        Foo::MkBool(x) => Ok(x),
                        other => Err(other)
                    }
                }
            }

            #[automatically_derived]
            impl ::sappho_fconv::Embed<bool> for (Foo) {
                fn embed(x: bool) -> Self {
                    Foo::MkBool(x)
                }
            }
        "# },
    }
    ; "concrete-enum-1-indexed-fields"
)]
fn extract_derive_pm2(tc: TestCase<&str>) {
    tc.check_expansion()
}

// Test Matrix Axes Types:
// Maybe TODO?
// enum FieldCount { Single, Multiple }

fn ident(s: &str) -> Ident {
    let tokenstream = s.parse().unwrap();
    syn::parse2(tokenstream).unwrap()
}

#[test_matrix(
    [Struct, Enum],
    [Indexed, Named(ident("my_field"))],
    [Concrete, Generic]
)]
fn matrix(kind: Kind, fspec: FieldSpec, gen: Genericity) {
    dbg!(TestCase {
        input: matrix_input(kind, fspec.clone(), gen),
        expected: matrix_expected(kind, fspec, gen),
    })
    .as_strs()
    .check_expansion()
}

fn matrix_input(kind: Kind, fspec: FieldSpec, gen: Genericity) -> String {
    let (kindkw, decl_open, decl_close) = match kind {
        Struct => ("struct ", "", ""),
        Enum => ("enum", "{ Thing", " }"),
    };

    let gparams = match gen {
        Concrete => "",
        Generic => "<T>",
    };

    let fdecl = match fspec {
        Indexed => "(T)".to_string(),
        Named(id) => format!("{{ {id} : T }}"),
    };

    let mut almost = format!("{kindkw} Foo{gparams} {decl_open}{fdecl}{decl_close}");
    if almost.ends_with(')') {
        almost.push(';');
    }
    almost
}

fn matrix_expected(kind: Kind, fspec: FieldSpec, gen: Genericity) -> String {
    format!(
        indoc! { r#"
            #[automatically_derived]
            impl {0} ::sappho_fconv::Extract<T> for (Foo {0}) {{
                fn extract(self) -> Result<T, Self> {{
                    match self {{
                        {1}{2} => Ok(x),
                        {3}
                    }}
                }}
            }}

            #[automatically_derived]
            impl {0} ::sappho_fconv::Embed<T> for (Foo {0}) {{
                fn embed(x: T) -> Self {{
                    {1}{2}
                }}
            }}
        "# },
        // Optional generic Params:
        match gen {
            Concrete => "",
            Generic => "<T>",
        },
        // Pattern / Construction path:
        match kind {
            Struct => "Foo",
            Enum => "Foo::Thing",
        },
        // Field pattern / constructor:
        match fspec {
            Indexed => "( x )".to_string(),
            Named(id) => format!("{{ {id} : x }}"),
        },
        // Optional enum fallthrough case:
        match kind {
            Struct => "",
            Enum => "other => Err(other)",
        }
    )
}

fn parse_tokens(s: &str) -> TokenStream {
    s.parse().unwrap()
}
