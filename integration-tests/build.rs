use indoc::indoc;
use pathutil::{FileTypeEnum, PathExt};
use std::fs::File;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/test-cases");
    let path = Path::new("src/gentests.rs");
    let mut f = File::create(path)?;
    generate_tests(&mut f)?;
    Ok(())
}

fn generate_tests(f: &mut File) -> Result<()> {
    for entry in Path::new("src/test-cases").pe_read_dir_entries()? {
        entry.metadata()?.require_file_type(FileTypeEnum::Dir)?;
        generate_case_tests(f, &entry.path())?;
    }

    Ok(())
}

fn generate_case_tests(f: &mut File, casedir: &Path) -> Result<()> {
    let casename = casedir.pe_file_name_str()?;
    let expected = casedir.join("expected");
    for entry in casedir.pe_read_dir_entries()? {
        entry.metadata()?.require_file_type(FileTypeEnum::File)?;
        let path = entry.path();
        let name = path.pe_file_name_str()?;
        if name == "input" || name.starts_with("input-") {
            generate_case_input_test(
                f,
                &expected,
                &path,
                &format!("{}_{}", casename, name).replace('-', "_"),
            )?;
        } else if name != "expected" {
            use std::io::{Error, ErrorKind::Other};
            return Err(Error::new(
                Other,
                format!("Unexpected file: {:?}", path.display()),
            ));
        }
    }

    Ok(())
}

fn generate_case_input_test(
    f: &mut File,
    expected: &Path,
    input: &Path,
    testname: &str,
) -> Result<()> {
    use std::io::Write;

    let exppath = expected.pe_strip_prefix("src/")?;
    let inpath = input.pe_strip_prefix("src/")?;
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
            testname,
            inpath.display(),
            inpath.display(),
            exppath.display(),
        )
        .as_bytes(),
    )
}
