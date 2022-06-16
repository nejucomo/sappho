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
        generate_test_case(f, &entry.path())?;
    }

    Ok(())
}

fn generate_test_case(f: &mut File, casedir: &Path) -> Result<()> {
    use std::io::Write;

    let testname = casedir.pe_file_name_str()?;
    let relcasedir = casedir.pe_strip_prefix("src/")?;
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
}
