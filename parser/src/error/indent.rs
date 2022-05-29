pub(crate) fn indent(prefix: &str, unindented: &str) -> String {
    let mut indented = String::new();
    for line in unindented.lines() {
        indented += prefix;
        indented += line;
        indented += "\n";
    }
    indented
}
