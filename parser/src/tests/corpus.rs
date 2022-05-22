mod error;

use self::error::{Error, Errors, Mismatch, Reason};
use include_dir::{include_dir, Dir};
use saplang_ast::PureExpr;

static CORPUS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/tests/corpus");

#[test]
fn positives() {
    parse_corpus("positives", parse_file)
}

#[test]
fn negatives() {
    parse_corpus("negatives", parse_file_negative)
}

fn parse_corpus<F, T>(corpusname: &str, parsefunc: F)
where
    F: Fn(&[u8]) -> Result<T, Reason>,
    T: ToString,
{
    if let Some(e) = parse_corpus_result(corpusname, parsefunc).err() {
        panic!("{}", e);
    }
}

fn parse_corpus_result<F, T>(corpusname: &str, parsefunc: F) -> Result<(), Errors>
where
    F: Fn(&[u8]) -> Result<T, Reason>,
    T: ToString,
{
    let mut errors = Errors::default();

    let corpcase = CORPUS_DIR
        .get_dir(corpusname)
        .unwrap_or_else(|| panic!("src/tests/corpus/{} not found", corpusname));

    for casedir in only_dirs(corpcase) {
        if let Some(reason) = parse_case(casedir, &parsefunc).err() {
            let casepath = casedir.path().to_path_buf();
            errors.push(Error(casepath, reason))
        }
    }

    errors.into_result()
}

fn only_dirs<'a>(d: &Dir<'a>) -> Vec<&'a Dir<'a>> {
    use include_dir::DirEntry::Dir;

    let mut ds = vec![];
    for entry in d.entries() {
        match entry {
            Dir(d) => ds.push(d),
            _ => panic!("Unexpected file: {:#?}", entry),
        }
    }
    ds
}

fn parse_case<F, T>(casedir: &Dir, parsefunc: F) -> Result<(), Reason>
where
    F: Fn(&[u8]) -> Result<T, Reason>,
    T: ToString,
{
    let input = file_contents(casedir, "input")?;
    match String::from_utf8(file_contents(casedir, "expected")?.to_vec()) {
        Ok(expected) => match parsefunc(input).map(|v| v.to_string()) {
            Ok(found) if found == expected => Ok(()),
            Ok(found) => Err(Reason::MismatchedOutput(Mismatch { found, expected })),
            Err(reason) => Err(reason),
        },
        Err(r) => Err(Reason::from(r)),
    }
}

fn file_contents<'a>(d: &'a Dir, fname: &'static str) -> Result<&'a [u8], Reason> {
    d.get_file(d.path().join(fname))
        .map(|f| f.contents())
        .ok_or_else(|| Reason::MissingFile(fname))
}

fn parse_file(srcbytes: &[u8]) -> Result<PureExpr, Reason> {
    let source = std::str::from_utf8(srcbytes)?;
    let expr = crate::parse(source)?;
    Ok(expr)
}

fn parse_file_negative(srcbytes: &[u8]) -> Result<crate::Errors, Reason> {
    match parse_file(srcbytes) {
        Ok(expr) => Err(Reason::InvalidParse(expr)),
        Err(Reason::Parse(errs)) => Ok(errs),
        Err(e) => Err(e),
    }
}
