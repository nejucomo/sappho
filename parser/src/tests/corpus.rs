use derive_more::From;
use include_dir::{include_dir, Dir};
use saplang_ast::PureExpr;

static CORPUS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/tests/corpus");

#[derive(Debug)]
struct Error(std::path::PathBuf, Reason);

#[derive(Debug, From)]
enum Reason {
    Utf8(std::str::Utf8Error),
    Parse(Vec<crate::Error>),
    InvalidParse(PureExpr),
}

#[test]
fn positives() -> Result<(), Vec<Error>> {
    parse_corpus("positives", parse_file)
}

#[test]
fn negatives() -> Result<(), Vec<Error>> {
    parse_corpus("negatives", parse_file_negative)
}

fn parse_corpus<F, T>(corpusname: &str, parsefunc: F) -> Result<(), Vec<Error>>
where
    F: Fn(&[u8]) -> Result<T, Reason>,
{
    let mut ferrors = vec![];

    let corpcase = CORPUS_DIR
        .get_dir(corpusname)
        .unwrap_or_else(|| panic!("src/tests/corpus/{} not found", corpusname));

    for f in corpcase.files() {
        let fpath = f.path().to_path_buf();
        if let Some(reason) = parsefunc(f.contents()).err() {
            ferrors.push(Error(fpath, reason));
        }
    }

    if ferrors.is_empty() {
        Ok(())
    } else {
        Err(ferrors)
    }
}

fn parse_file(srcbytes: &[u8]) -> Result<PureExpr, Reason> {
    let source = std::str::from_utf8(srcbytes)?;
    let expr = crate::parse(source)?;
    Ok(expr)
}

fn parse_file_negative(srcbytes: &[u8]) -> Result<(), Reason> {
    match parse_file(srcbytes) {
        Ok(expr) => Err(Reason::InvalidParse(expr)),
        Err(Reason::Parse(_)) => Ok(()),
        Err(e) => Err(e),
    }
}
