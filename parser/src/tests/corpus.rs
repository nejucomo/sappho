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
    let expected = file_contents(casedir, "expected")
        .map_err(|e| Errors::from([e]))?
        .trim_end();

    let mut inputs: Vec<&File> = vec![];
    for f in casedir.files() {
        if f.path()
            .file_name()
            .and_then(|os| os.to_str())
            .ok_or(Reason::BadPath)
            .map(|fname| fname.starts_with("input"))
            .map_err(|r| Errors::from([Error(f.path().to_path_buf(), r)]))?
        {
            inputs.push(f);
        }
    }

    if inputs.is_empty() {
        return Err(Errors::from([Error(
            casedir.path().to_path_buf(),
            Reason::MissingFile("<no 'input*' files for case directory>"),
        )]));
    } else {
        let mut errors = Errors::default();

        for f in inputs {
            errors.track_error(parse_case_input(f, expected, &parsefunc));
        }

        errors.into_result()
    }
}

fn parse_case_input<F, T>(inputfile: &File, expected: &str, parsefunc: F) -> Result<(), Error>
where
    F: Fn(PathBuf, &str) -> Result<T, Reason>,
    T: ToString,
{
    parse_case_input_reason(inputfile, expected, &parsefunc)
        .map_err(|r| Error(inputfile.path().to_path_buf(), r))
}

fn parse_case_input_reason<F, T>(
    inputfile: &File,
    expected: &str,
    parsefunc: F,
) -> Result<(), Reason>
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

fn file_contents<'a>(d: &'a Dir, fname: &'static str) -> Result<&'a str, Error> {
    file_contents_to_reason(d, fname).map_err(|r| Error(d.path().join(fname), r))
}

fn file_contents_to_reason<'a>(d: &'a Dir, fname: &'static str) -> Result<&'a str, Reason> {
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
