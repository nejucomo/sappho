use anyhow::Result;
use anyhow_std::PathAnyhow;
use indoc::indoc;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/test-cases");
    let path = Path::new("src/gentests.rs");
    let mut f = File::create(path)?;
    generate_tests(&mut f)?;
    Ok(())
}

fn generate_tests(f: &mut File) -> Result<()> {
    for entres in Path::new("src/test-cases").read_dir_anyhow()? {
        let entry = entres?;

        if entry.metadata()?.is_dir() {
            generate_case_tests(f, &entry.path())?;
        } else {
            anyhow::bail!("expected a dir, found {:?}", entry);
        }
    }

    Ok(())
}

fn generate_case_tests(f: &mut File, casedir: &Path) -> Result<()> {
    use anyhow_std::OsStrAnyhow;

    let casename = casedir.file_name_anyhow()?.to_str_anyhow()?;
    let expected = casedir.join("expected");
    let mut has_canonical = false;
    let mut has_reduced = false;
    let mut inputs = vec![];
    for entres in casedir.read_dir_anyhow()? {
        let entry = entres?;
        if !entry.metadata()?.is_file() {
            anyhow::bail!("expected a file, found {:?}", entry);
        }

        let path = entry.path();
        let name = path.file_name_anyhow()?.to_str_anyhow()?;
        if name == "input" || name.starts_with("input-") {
            let inputcasename = format!("{}_{}", casename, name).replace('-', "_");
            generate_case_input_test(f, &expected, &path, &inputcasename)?;

            if name == "input-canonical" {
                has_canonical = true;
            } else if name == "input-reduced" {
                has_reduced = true;
            }

            inputs.push((inputcasename, path));
        } else if name != "expected" {
            anyhow::bail!("Unexpected file: {:?}", path.display());
        }
    }

    if has_canonical && has_reduced {
        for (icname, inpath) in inputs.iter() {
            generate_unparse_case(f, casedir, inpath, icname, "canonical")?;
            generate_unparse_case(f, casedir, inpath, icname, "reduced")?;
        }
        Ok(())
    } else if !has_canonical && !has_reduced {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Inconsistent 'input-canonical' vs 'input-reduced' in {:?}",
            casedir.display()
        ))
    }
}

fn generate_case_input_test(
    f: &mut File,
    expected: &Path,
    input: &Path,
    testname: &str,
) -> Result<()> {
    use std::io::Write;

    let exppath = expected.strip_prefix("src/")?;
    let inpath = input.strip_prefix("src/")?;
    f.write_all(
        format!(
            indoc! {r#"
                #[test]
                fn {testname}() {{
                    let inpath = std::path::PathBuf::from("{inpath}");
                    let input = include_str!("{inpath}");
                    let expected = include_str!("{exppath}");
                    crate::test_eval(inpath, input, expected);
                }}
            "#},
            testname = testname,
            inpath = inpath.display(),
            exppath = exppath.display(),
        )
        .as_bytes(),
    )?;
    Ok(())
}

fn generate_unparse_case(
    f: &mut File,
    casedir: &Path,
    input: &Path,
    icname: &str,
    style: &str,
) -> Result<()> {
    use std::io::Write;

    let exppathhost = casedir.join(format!("input-{}", style));
    let exppath = exppathhost.strip_prefix("src/")?;
    let inpath = input.strip_prefix("src/")?;
    f.write_all(
        format!(
            indoc! {r#"
                #[test]
                fn unparse_{style}_{icname}() {{
                    let inpath = std::path::PathBuf::from("{inpath}");
                    let input = include_str!("{inpath}");
                    let expected = include_str!("{exppath}");
                    crate::test_unparse(inpath, input, expected, {style:?});
                }}
            "#},
            style = style,
            icname = icname,
            inpath = inpath.display(),
            exppath = exppath.display(),
        )
        .as_bytes(),
    )?;
    Ok(())
}
