// rules interface here with one function that returns a line result or nothing
pub struct Rule {}
// also uses these structs to build the line result
#[derive(Clone)]
pub struct LineResult {
    pub severity: Severity,
    pub line: i32,
    pub column: i32,
    pub classification: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Warning,
    Info,
}
impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
