use chumsky::primitive::filter;
use chumsky::Parser as _;
use either::Either::{self, Left, Right};
use indoc::indoc;
use sappho_syntax_parsable::{Parsable, ParsableWith};
use sappho_syntax_unparse::{Stream, Unparse};
use test_case::test_case;

use crate::ListForm;

// A fake element for parsing:
#[derive(Debug, derive_more::From, derive_more::Into)]
struct TestElem(char);

impl ParsableWith<()> for TestElem {
    fn make_parser_with(_: ()) -> impl sappho_syntax_parsable::Parser<Self> {
        filter::<char, _, _>(|&c| c.is_ascii_alphabetic()).map(TestElem)
    }
}

impl Unparse for TestElem {
    fn unparse_into(&self, _: &mut Stream) {
        unimplemented!("not exercised by test code");
    }
}

#[test_case("[]" => Ok((vec![], None)))]
#[test_case("[x]" => Ok((vec!['x'], None)))]
#[test_case("[x, y]" => Ok((vec!['x', 'y'], None)))]
#[test_case("[x, y, z]" => Ok((vec!['x', 'y', 'z'], None)))]
#[test_case("[..z]" => Ok((vec![], Some('z'))))]
#[test_case("[x, ..z]" => Ok((vec!['x'], Some('z'))))]
#[test_case("[x, y, ..z]" => Ok((vec!['x', 'y'], Some('z'))))]
// Notice the chumsky error bug which we are matching:
#[test_case("[x, ..z, y]" => Err("Parse errors in <memory>: found end of input".to_string()))]
// White space coverage:
#[test_case("[\n]" => Ok((vec![], None)); "empty-newline")]
#[test_case("[ \n ]" => Ok((vec![], None)); "empty-newline-space-sandwich")]
#[test_case("[ x]" => Ok((vec!['x'], None)); "space-prefix-solo-item")]
#[test_case("[x ]" => Ok((vec!['x'], None)); "space-suffix-solo-item")]
#[test_case("[\n  x\n]" => Ok((vec!['x'], None)); "natural-newlines-solo-item")]
#[test_case("[ x , y , ..z ]" => Ok((vec!['x', 'y'], Some('z'))); "spaces-everywhere-pair-with-tail")]
fn parse(input: &str) -> Result<(Vec<char>, Option<char>), String>
where
{
    ListForm::<TestElem, TestElem>::load_and_parse(input)
        .map(ListForm::into)
        .map(|(v, opt)| (v.into_iter().map(char::from).collect(), opt.map(char::from)))
        .map_err(|e| e.to_string())
}

#[test_case(["x", "y"], None, ["x", "y"])]
#[test_case(["x", "y"], Some("z"), ["x", "y", "z"])]
fn test_iter<I, const K: usize>(
    xs: I,
    optail: Option<&'static str>,
    mut expected: [&'static str; K],
) where
    I: IntoIterator<Item = &'static str>,
{
    let lf = ListForm::try_from_iter(
        xs.into_iter()
            .map(Left)
            .chain(optail.into_iter().map(Right)),
    )
    .unwrap();

    let v: Vec<_> = lf.clone().into_iter().map(Either::into_inner).collect();
    assert_eq!(v.as_slice(), expected.as_slice());

    let v: Vec<_> = lf.into_iter().rev().map(Either::into_inner).collect();
    expected.as_mut_slice().reverse();
    assert_eq!(v.as_slice(), expected.as_slice());
}

struct X;

impl Unparse for X {
    fn unparse_into(&self, s: &mut Stream) {
        s.write("X");
    }
}

#[test_case([], None => "[]")]
#[test_case([], Some(X) => indoc! { "
        [
          ..X
        ]"
    })]
#[test_case([X], None => indoc! { "
        [
          X
        ]"
    })]
#[test_case([X], Some(X) => indoc! { "
        [
          X,
          ..X
        ]"
    })]
#[test_case([X, X], Some(X) => indoc! { "
        [
          X,
          X,
          ..X
        ]"
    })]
fn display<const K: usize>(body: [X; K], tail: Option<X>) -> String {
    ListForm::new(body, tail).to_string()
}
