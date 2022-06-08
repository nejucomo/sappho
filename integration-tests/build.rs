use indoc::indoc;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug)]
#[allow(dead_code)] // Used in Debug error display.
struct PathError {
    path: PathBuf,
    error: std::io::Error,
}

type Result<T> = std::result::Result<T, PathError>;

/// A trait for extending std::io::Result w/ path annotations.
trait WrapPathError<T> {
    fn add_error_path(self, path: &Path) -> Result<T>;
}

impl<T> WrapPathError<T> for std::io::Result<T> {
    fn add_error_path(self, path: &Path) -> Result<T> {
        self.map_err(|error| PathError {
            path: path.to_path_buf(),
            error,
        })
    }
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/test-cases");
    let path = Path::new("src/gentests.rs");
    let mut f = File::create(path).add_error_path(path)?;
    generate_tests(&mut f)?;
    Ok(())
}

fn generate_tests(f: &mut File) -> Result<()> {
    for_each_dir_entry("src/test-cases", |entry| {
        let ftype = entry.file_type().add_error_path(&entry.path())?;
        if ftype.is_dir() {
            generate_test_case(f, &entry.path())
        } else {
            Err(PathError {
                path: entry.path(),
                error: ioerror(format!("Unexpected: {:?}", entry.path())),
            })
        }
    })
}

fn generate_test_case(f: &mut File, casedir: &Path) -> Result<()> {
    use std::io::Write;

    let testname = file_name(casedir)?;
    let relcasedir = casedir.strip_prefix("src/").unwrap();
    let inpath = relcasedir.join("input");
    let inpathdisp = inpath.display();
    f.write_all(
        format!(
            indoc! {r#"
                #[test]
                fn {}() {{
                    let inpath = std::path::PathBuf::from("{}");
                    let input = include_str!("{}");
                    let expected = include_str!("{}");
                    crate::test_eval(inpath, input, expected);
                }}
            "#},
            testname.replace('-', "_"),
            inpathdisp,
            inpathdisp,
            relcasedir.join("expected").display(),
        )
        .as_bytes(),
    )
    .add_error_path(casedir)
}

fn for_each_dir_entry<P, F>(dir: P, mut f: F) -> Result<()>
where
    P: AsRef<Path>,
    F: FnMut(std::fs::DirEntry) -> Result<()>,
{
    let dref = dir.as_ref();
    for entres in std::fs::read_dir(dref).add_error_path(dref)? {
        let entry = entres.add_error_path(dref)?;
        f(entry)?;
    }
    Ok(())
}

fn file_name(path: &Path) -> Result<&str> {
    file_name_stdio(path).add_error_path(path)
}

fn file_name_stdio(path: &Path) -> std::io::Result<&str> {
    let osstr = path
        .file_name()
        .ok_or_else(|| ioerror("No filename".to_string()))?;
    osstr
        .to_str()
        .ok_or_else(|| ioerror("Non-utf8 filename".to_string()))
}

fn ioerror(msg: String) -> std::io::Error {
    use std::io::Error;
    use std::io::ErrorKind::Other;

    Error::new(Other, msg)
}
