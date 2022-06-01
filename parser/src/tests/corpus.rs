mod error;

use self::error::{Error, Errors, Mismatch, Reason};
use include_dir::{include_dir, Dir, File};
use sappho_ast::PureExpr;
use std::path::PathBuf;

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
    F: Fn(PathBuf, &str) -> Result<T, Reason>,
    T: ToString,
{
    if let Some(e) = parse_corpus_result(corpusname, parsefunc).err() {
        panic!("{}", e);
    }
}

fn parse_corpus_result<F, T>(corpusname: &str, parsefunc: F) -> Result<(), Errors>
where
    F: Fn(PathBuf, &str) -> Result<T, Reason>,
    T: ToString,
{
    let mut errors = Errors::default();

    let corpcase = CORPUS_DIR
        .get_dir(corpusname)
        .unwrap_or_else(|| panic!("src/tests/corpus/{} not found", corpusname));

    for casedir in only_dirs(corpcase) {
        if let Some(suberrs) = parse_case(casedir, &parsefunc).err() {
            errors.extend(suberrs);
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

fn parse_case<F, T>(casedir: &Dir, parsefunc: F) -> Result<(), Errors>
where
    F: Fn(PathBuf, &str) -> Result<T, Reason>,
    T: ToString,
{
    let mut errors = Errors::default();

    let expected = match file_contents(casedir, "expected") {
        Ok(x) => x.trim_end(),
        Err(r) => {
            errors.push(Error(casedir.path().join("expected"), r));
            return Err(errors);
        }
    };

    for f in casedir.files() {
        let fpath = f.path();
        let fname = fpath.strip_prefix(casedir.path()).unwrap();
        if fname.starts_with("input") {
            if let Some(reason) = parse_case_input(f, expected, &parsefunc).err() {
                errors.push(Error(fpath.to_path_buf(), reason))
            }
        }
    }

    errors.into_result()
}

fn parse_case_input<F, T>(inputfile: &File, expected: &str, parsefunc: F) -> Result<(), Reason>
where
    F: Fn(PathBuf, &str) -> Result<T, Reason>,
    T: ToString,
{
    let input = std::str::from_utf8(inputfile.contents())?;
    let inpath = inputfile.path().to_path_buf();

    match parsefunc(inpath, input).map(|v| v.to_string()) {
        Ok(found) if found.trim_end() == expected => Ok(()),

        // Allow a missing "expected" file as a dev convenience:
        Ok(found) => Err(Reason::MismatchedOutput(Mismatch {
            found: found.trim_end().to_string(),
            expected: expected.to_string(),
        })),

        Err(reason) => Err(reason),
    }
}

fn file_contents<'a>(d: &'a Dir, fname: &'static str) -> Result<&'a str, Reason> {
    let bytes = d
        .get_file(d.path().join(fname))
        .map(|f| f.contents())
        .unwrap_or(b"");
    let src = std::str::from_utf8(bytes)?;
    Ok(src)
}

fn parse_file(path: PathBuf, source: &str) -> Result<PureExpr, Reason> {
    let expr = crate::parse(Some(path), source)?;
    Ok(expr)
}

fn parse_file_negative(path: PathBuf, source: &str) -> Result<crate::Errors, Reason> {
    match parse_file(path, source) {
        Ok(expr) => Err(Reason::InvalidParse(expr)),
        Err(Reason::Parse(errs)) => Ok(errs),
        Err(e) => Err(e),
    }
}
