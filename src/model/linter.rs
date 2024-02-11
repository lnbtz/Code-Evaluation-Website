// parses the lines and returns the linter result
// LinterResult is a struct that contains the result of the linter
// each line is parsed and the result is stored in the LinterResult
use crate::model::rules::*;

pub fn parse_code(code: String, file_type: String, rules_to_apply: Vec<String>) -> Vec<LineResult> {
    let mut line_results: Vec<LineResult> = Vec::new();
    // use rules based on file_type
    let rules: Vec<Box<dyn Rule>> = load_rules(file_type, rules_to_apply);

    for rule in rules {
        if let Some(result) = rule.apply(&code) {
            line_results.extend(result);
        }
    }
    line_results
}
