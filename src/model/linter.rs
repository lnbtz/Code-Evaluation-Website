// parses the lines and returns the linter result
// LinterResult is a struct that contains the result of the linter
// each line is parsed and the result is stored in the LinterResult
use crate::model::rules::*;

/// parse_code parses the code and returns the linter result
pub fn evalute_code(code: &str, file_type: String, rules_to_apply: Vec<String>) -> Vec<LineResult> {
    let mut line_results: Vec<LineResult> = Vec::new();
    // load rules based on file_type and seleted rules
    let rules: Vec<Box<dyn Rule>> = load_rules(file_type, rules_to_apply);
    // apply rules to the code and store the results
    for rule in rules {
        if let Some(result) = rule.apply(code) {
            line_results.extend(result);
        }
    }
    if line_results.is_empty() {
        line_results.push(LineResult {
            severity: Severity::Info,
            line: 0,
            column: 0,
            classification: "info".to_string(),
            description: "No issues found".to_string(),
        });
    }
    line_results
}
