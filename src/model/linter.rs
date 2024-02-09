// parses the lines and returns the linter result
// LinterResult is a struct that contains the result of the linter
// each line is parsed and the result is stored in the LinterResult
use crate::model::rules::*;
pub fn parse_code(code: String, file_type: String) -> Vec<LineResult> {
    let mut line_results: Vec<LineResult> = Vec::new();
    // use rules based on file_type
    let rules: Vec<Rule> = match file_type.as_str() {
        "html" => {
            // Add rules for html files
            load_html_rules()
        }
        "css" => {
            // Add rules for css files
            load_css_rules()
        }
        "js" => {
            // Add rules for js files
            load_js_rules()
        }
        // Add more cases for other file types
        _ => {
            // return error for unknown file type #TODO
            todo!("Unknown file type")
        }
    };

    // add rule listeners here that return a line result or nothing
    // maybe use interface for rules that has one function that returns a line result or nothing
    // then add all rules to a list and call the function on each rule #TODO

    code.split('\n').enumerate().for_each(|(line_nr, line)| {
        if !line.is_empty() {
            let severity = Severity::Warning;
            let column = 0;
            let classification = "Line".to_string();
            let description = line.to_string();
            let line_result = LineResult {
                severity,
                line: line_nr as i32 + 1,
                column,
                classification,
                description,
            };
            line_results.push(line_result);
        }
    });
    line_results
}
