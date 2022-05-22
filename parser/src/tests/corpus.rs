use derive_more::From;
use include_dir::{include_dir, Dir};
use saplang_ast::PureExpr;

static CORPUS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/tests/corpus");

#[derive(Debug)]
struct Error(std::path::PathBuf, Reason);

#[derive(Debug, From)]
enum Reason {
    MissingFile(&'static str),
    StrUtf8(std::str::Utf8Error),
    StringUtf8(std::string::FromUtf8Error),
    Parse(Vec<crate::Error>),
    InvalidParse(PureExpr),
    MismatchedOutput(String, String),
}

#[test]
fn positives() -> Result<(), &'static str> {
    parse_corpus_pretty("positives", parse_file)
}

#[test]
fn negatives() -> Result<(), &'static str> {
    parse_corpus_pretty("negatives", parse_file_negative)
}

fn parse_corpus_pretty<F, T>(corpusname: &str, parsefunc: F) -> Result<(), &'static str>
where
    F: Fn(&[u8]) -> Result<T, Reason>,
    T: TestFormat,
{
    parse_corpus(corpusname, parsefunc).map_err(|es| {
        for Error(casepath, reason) in es {
            eprintln!("Error in case {:?}: {:#?}", casepath, reason);
        }
        "some cases had errors"
    })
}

fn parse_corpus<F, T>(corpusname: &str, parsefunc: F) -> Result<(), Vec<Error>>
where
    F: Fn(&[u8]) -> Result<T, Reason>,
    T: TestFormat,
{
    let mut ferrors = vec![];

    let corpcase = CORPUS_DIR
        .get_dir(corpusname)
        .unwrap_or_else(|| panic!("src/tests/corpus/{} not found", corpusname));

    for casedir in only_dirs(corpcase) {
        let casepath = casedir.path().to_path_buf();
        if let Some(reason) = parse_case(casedir, &parsefunc).err() {
            ferrors.push(Error(casepath, reason))
        }
    }

    if ferrors.is_empty() {
        Ok(())
    } else {
        Err(ferrors)
    }
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
    T: TestFormat,
{
    let input = file_contents(casedir, "input")?;
    match String::from_utf8(file_contents(casedir, "expected")?.to_vec()) {
        Ok(expected) => match parsefunc(input).map(|v| v.test_format()) {
            Ok(found) if found == expected => Ok(()),
            Ok(found) => Err(Reason::MismatchedOutput(found, expected)),
            Err(reason) => Err(reason),
        },
        Err(r) => Err(Reason::from(r)),
    }
}

fn file_contents<'a>(d: &'a Dir, fname: &'static str) -> Result<&'a [u8], Reason> {
    d.get_file(fname)
        .ok_or_else(|| Reason::MissingFile(fname))
        .map(|f| f.contents())
}

fn parse_file(srcbytes: &[u8]) -> Result<PureExpr, Reason> {
    let source = std::str::from_utf8(srcbytes)?;
    let expr = crate::parse(source)?;
    Ok(expr)
}

fn parse_file_negative(srcbytes: &[u8]) -> Result<Vec<crate::Error>, Reason> {
    match parse_file(srcbytes) {
        Ok(expr) => Err(Reason::InvalidParse(expr)),
        Err(Reason::Parse(errs)) => Ok(errs),
        Err(e) => Err(e),
    }
}

trait TestFormat {
    fn test_format(self) -> String;
}

impl TestFormat for Vec<crate::Error> {
    fn test_format(self) -> String {
        self.into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl TestFormat for PureExpr {
    fn test_format(self) -> String {
        format!("{:#?}", self)
    }
}
